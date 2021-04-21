mod tera_engine;

use std::iter::FromIterator;
use std::{collections::HashMap, process::Stdio};

use fluent::bundle::FluentBundle;
use fluent_bundle::{FluentArgs, FluentResource, FluentValue};
use intl_memoizer::concurrent::IntlLangMemoizer;
use pulldown_cmark::{html, Options, Parser};
use rocket::{
    fairing::{Fairing, Info, Kind},
    get,
    http::{ContentType, Header},
    launch, options, post, routes, Request, Response,
};
use rocket_contrib::{json::Json, serve::StaticFiles};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    process::{ChildStdout, Command},
};

#[get("/get/<invoice>/<version>")]
async fn get<'a>(
    invoice: String,
    version: Option<String>,
) -> Result<Response<'a>, tokio::io::Error> {
    let data = tokio::fs::File::open(format!(
        "invoices/{}/{}/{}",
        invoice,
        version.unwrap_or("0".into()),
        "invoice.pdf"
    ));

    let response = rocket::Response::build()
        .header(ContentType::new("application", "pdf"))
        .streamed_body(data.await?)
        .ok();

    return response;
}

#[derive(Serialize)]
struct List {
    invoices: Vec<Invoice>,
}

#[derive(Serialize)]
struct Invoice {
    versions: Vec<String>,
    name: String,
}

#[get("/list")]
async fn list<'a>() -> Result<Json<List>, tokio::io::Error> {
    let mut entries = std::fs::read_dir("invoices")
        .unwrap()
        .map(|e| e.unwrap())
        .collect::<Vec<_>>();
    entries.sort_by_key(|e| e.path());

    let mut invoices = Vec::new();
    for entry in entries {
        let mut versions = Vec::new();
        let mut entries = std::fs::read_dir(entry.path())
            .unwrap()
            .map(|e| e.unwrap())
            .collect::<Vec<_>>();
        entries.sort_by_key(|e| e.path());
        for entry in entries {
            versions.push(entry.file_name().to_string_lossy().to_string());
        }
        invoices.push(Invoice {
            versions,
            name: entry.file_name().to_string_lossy().into(),
        });
    }

    Ok(Json(List { invoices }))
}

#[post("/generate", data = "<data>")]
async fn generate<'a>(data: Json<GenerateData>) -> Result<Response<'a>, tokio::io::Error> {
    let stdout = generate_pdf(&data.0).await?;
    println!("{}", stdout.0);
    let response = rocket::Response::build()
        .header(ContentType::new("application", "pdf"))
        .streamed_body(stdout.1)
        .ok();

    return response;
}

#[post("/store", data = "<data>")]
async fn store<'a>(data: Json<GenerateData>) -> Result<(), tokio::io::Error> {
    let mut stdout = generate_pdf(&data.0).await?;

    let dir = if let Ok(entries) = std::fs::read_dir(format!("invoices/{}", data.0.no)) {
        let mut entries: Vec<_> = entries.map(|e| e.unwrap()).collect();
        entries.sort_by_key(|e| e.path());
        let v: usize = entries[entries.len() - 1]
            .file_name()
            .to_string_lossy()
            .to_string()
            .parse()
            .unwrap();

        let dir = format!("invoices/{}/{}", data.0.no, v + 1);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    } else {
        let dir = format!("invoices/{}/0", data.0.no);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    };

    let mut bytes = Vec::new();
    stdout.1.read_to_end(&mut bytes).await?;

    std::fs::write(format!("{}/{}", dir, "invoice.pdf"), bytes).unwrap();
    let json = serde_json::to_string_pretty(&data.0).unwrap();
    std::fs::write(format!("{}/{}", dir, "data.json"), json).unwrap();

    Ok(())
}

#[options("/generate")]
async fn generate_preflight() -> Result<(), ()> {
    Ok(())
}

#[options("/store")]
async fn store_preflight() -> Result<(), ()> {
    Ok(())
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'a>(&self, _request: &'a Request<'_>, response: &mut Response<'a>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS)
        .mount(
            "/",
            routes![
                // index,
                get,
                generate,
                store,
                generate_preflight,
                store_preflight,
                list
            ],
        )
        .mount("/", StaticFiles::from("frontend/dist"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateData {
    language: String,
    date: String,
    due: String,
    title: String,
    address: String,
    no: String,
    contact: String,
    reference: String,
    text: String,
    positions: Vec<Position>,
    currency: String,
    vat_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    id: usize,
    text: String,
    count: f64,
    cost: f64,
}

async fn generate_pdf(
    data: &GenerateData,
) -> Result<(std::process::ExitStatus, ChildStdout), tokio::io::Error> {
    println!("{:#?}", data);

    let total: f64 = data.positions.iter().map(|p| p.count * p.cost).sum();
    let vat = total * data.vat_rate / 100.0;

    let translations = load_translation(&data.language);
    // here "ipinfo::Response" need also be changed to "ip2asn::Response" for free api calls
    let mut context = tera::Context::new();
    context.insert("data", data);

    let mut tera = tera::Tera::new("templates/*.tera").unwrap();

    {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_TABLES);
        let parser = Parser::new_ext(&data.text, options);

        let mut body = String::new();
        html::push_html(&mut body, parser);
        // context.insert("html_body", &body);
        tera.add_raw_template("body.html", &body).unwrap();
    }

    context.insert("total", &total);
    context.insert("vat", &vat);

    tera.register_function(
        "t",
        Box::new(
            move |args: &HashMap<std::string::String, serde_json::Value>| {
                let n = args["n"].as_str().unwrap();
                let args =
                    FluentArgs::from_iter(args.iter().map::<(String, FluentValue), _>(|(k, v)| {
                        (k.clone(), From::from(v.as_str().unwrap()))
                    }));
                translations
                    .get_message(n)
                    .and_then(|s| s.value())
                    .map(|s| {
                        serde_json::Value::String(
                            translations
                                .format_pattern(s, Some(&args), &mut vec![])
                                .into(),
                        )
                    })
                    .ok_or(tera::Error::msg(format!("{} was not found!", n)))
            },
        ),
    );

    let render = tera.render("template.html.tera", &context).unwrap();

    // execute weasyprint to generate pdf
    let mut weasyprint = Command::new("python3")
        .args(&["-m", "weasyprint"])
        .args(&["-f", "pdf"])
        .args(&["-e", "utf8"])
        .arg("-")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("failed to execute process");

    if let Some(mut stdin) = weasyprint.stdin.as_mut() {
        let mut writer = BufWriter::new(&mut stdin);
        writer.write_all(&render.into_bytes()).await?;
        writer.flush().await?;
    }

    let exit_status = weasyprint.wait().await?;
    Ok((exit_status, weasyprint.stdout.expect("This is a bug.")))
}

fn load_translation(language: &str) -> FluentBundle<FluentResource, IntlLangMemoizer> {
    let mut bundle = FluentBundle::new_concurrent(vec![language.parse().unwrap()]);

    let ftl_string = std::fs::read_to_string("templates/translations/en-US.ftl")
        .expect("Failed to open translation file.");

    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");
    bundle.add_resource(res).unwrap();

    let ftl_string = std::fs::read_to_string(format!("templates/translations/{}.ftl", language))
        .expect("Failed to open translation file.");
    let res = FluentResource::try_new(ftl_string).expect("Failed to parse an FTL string.");
    bundle.add_resource_overriding(res);

    bundle
}
