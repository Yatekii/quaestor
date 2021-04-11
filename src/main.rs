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
    launch, options, post,
    response::Redirect,
    routes, Request, Response, Rocket,
};
use rocket_contrib::{json::Json, serve::StaticFiles};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    process::{ChildStdout, Command},
};

#[get("/")]
fn index() -> Redirect {
    Redirect::to("/index.html")
}

#[get("/get/<hash>")]
async fn get<'a>(hash: String) -> Result<Response<'a>, tokio::io::Error> {
    let json = base64::decode(hash).unwrap();
    println!("{}", String::from_utf8_lossy(&json));
    let json = serde_json::from_slice(&json).unwrap();
    let stdout = generate_pdf(&json).await?;
    println!("{}", stdout.0);
    let response = rocket::Response::build()
        .header(ContentType::new("application", "pdf"))
        .streamed_body(stdout.1)
        .ok();

    return response;
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

#[options("/generate")]
async fn generate_preflight() -> Result<(), ()> {
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
fn rocket() -> Rocket {
    rocket::ignite()
        .attach(CORS)
        .mount("/", routes![index, get, generate, generate_preflight])
        .mount("/", StaticFiles::from("frontend/public"))
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
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    id: usize,
    text: String,
    count: f64,
    cost: f64,
    vat_included: bool,
    vat_must: bool,
}

async fn generate_pdf(
    data: &GenerateData,
) -> Result<(std::process::ExitStatus, ChildStdout), tokio::io::Error> {
    println!("{:?}", data);

    let total: f64 = data.positions.iter().map(|p| p.count * p.cost).sum();
    let vat = total * 0.077;

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
