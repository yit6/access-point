mod ap;
mod user;

#[macro_use] extern crate rocket;

use std::{fs, path::{PathBuf, Path}};
use rocket::{http::Status, fs::NamedFile, serde::json::Json};
use web_push::*;
use base64;

const FRONTEND_LOCATION: &str = "../access-point-ui/dist/";

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(FRONTEND_LOCATION).join("index.html")).await.ok()
}

#[get("/<file..>", rank = 6)]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(FRONTEND_LOCATION).join(file)).await.ok()
}

#[get("/backend-msg")]
async fn backend_msg() -> String {
    "Rocket backend!".to_string()
}

#[post("/save-subscription", data="<input>")]
async fn save_subscription(input: Json<SubscriptionInfo> /*notif handle here*/) -> (Status, String) {
    let file = fs::read_to_string("./secrets/private.txt").unwrap();
    println!("{:?}", &input);

    let mut sig_builder = VapidSignatureBuilder::from_base64(&file, URL_SAFE_NO_PAD, &input).unwrap()
        .build().unwrap();

    let mut builder = WebPushMessageBuilder::new(&input);
    let content = "Encrypted payload to be sent in the notification".as_bytes();
    builder.set_payload(ContentEncoding::Aes128Gcm, content);
    builder.set_vapid_signature(sig_builder);

    let client = IsahcWebPushClient::new().unwrap();

    client.send(builder.build().unwrap()).await.unwrap();
    (Status::Ok, serde_json::json!("Subscribed!").to_string())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, backend_msg, files, save_subscription])
        .attach(user::stage())
        .attach(ap::stage())
}
