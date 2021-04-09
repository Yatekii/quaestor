mod tera_engine;

use std::process::Stdio;

use chrono::{Datelike, TimeZone, Timelike, Utc};
use rocket::{get, http::ContentType, launch, post, response::Redirect, routes, Response, Rocket};
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
async fn get<'a>(hash: String) -> Result<Response<'a>, ()> {
    let json = base64::decode(hash).unwrap();
    let json = serde_json::from_slice(&json).unwrap();
    let stdout = generate_pdf(&json).await;
    let response = rocket::Response::build()
        .header(ContentType::new("application", "pdf"))
        .streamed_body(stdout)
        .ok();
    return response;
}

#[launch]
fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![index, get])
        .mount("/", StaticFiles::from("frontend/public"))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateData {
    date: String,
    title: String,
    positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    text: String,
    count: f64,
    cost: f64,
    curency: String,
    vat_included: bool,
    vat_must: bool,
}

async fn generate_pdf(data: &GenerateData) -> ChildStdout {
    println!("{:?}", data);
    // here "ipinfo::Response" need also be changed to "ip2asn::Response" for free api calls
    let mut context = tera::Context::new();
    context.insert("data", data);
    let render = crate::tera_engine::TERA
        .render("template.html.tera", &context)
        .unwrap();

    // execute weasyprint to generate pdf
    let weasyprint = Command::new("python3")
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

    let mut stdin = weasyprint.stdin.unwrap();
    let stdout = weasyprint.stdout.unwrap();
    let mut writer = BufWriter::new(&mut stdin);

    writer.write_all(&render.into_bytes()).await.unwrap();

    stdout
}
