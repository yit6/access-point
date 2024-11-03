mod ap;
mod user;

#[macro_use] extern crate rocket;

use std::{fs, path::{PathBuf, Path}, sync::{Arc, Mutex}};
use rocket::{http::Status, fs::NamedFile, serde::json::Json, State};
use web_push::*;
use base64;
use base64::{engine::general_purpose, Engine as _};

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

#[derive(Debug)]
struct NotificationServer {
    signature: Arc<Mutex<Option<VapidSignature>>>,
    notification_to_send: Arc<Mutex<Option<String>>>,
    sub: Arc<Mutex<Option<SubscriptionInfo>>>
}

impl NotificationServer {
    fn new() -> Self {
        NotificationServer {
            signature: Arc::new(Mutex::new(None)),
            notification_to_send: Arc::new(Mutex::new(None)),
            sub: Arc::new(Mutex::new(None)),
        }
    }

    fn set(&self, sig: VapidSignature, sub: SubscriptionInfo) {
        let signature = Arc::clone(&self.signature);
        let mut signature = signature.lock().unwrap();
        let subscription = Arc::clone(&self.sub);
        let mut subscription = subscription.lock().unwrap();
        *signature = Some(sig);
        *subscription = Some(sub);
    }

    fn notification(&self, notification: &str) {
        let notification_to_send = Arc::clone(&self.notification_to_send);
        let mut notification_to_send = notification_to_send.lock().unwrap();
        *notification_to_send = Some(notification.to_string());
    }

    fn get_notification(&self) -> String {
        let notification_to_send = Arc::clone(&self.notification_to_send);
        let notification_to_send = notification_to_send.lock().unwrap();
        notification_to_send.clone().unwrap().clone()
    }

    fn get_signature(&self) -> VapidSignature {
        let signature = Arc::clone(&self.signature);
        let signature = signature.lock().unwrap();
        signature.clone().unwrap().clone()
    }

    fn get_sub(&self) -> SubscriptionInfo {
        let sub = Arc::clone(&self.sub);
        let sub = sub.lock().unwrap();
        sub.clone().unwrap()
    }
}

#[post("/save-subscription", data="<input>")]
async fn save_subscription(input: Json<SubscriptionInfo>, serve: &State<NotificationServer>) -> (Status, String) {
    let file = fs::read_to_string("./secrets/private.txt").unwrap();

    println!("{:?}", &input);

    serve.set(
        VapidSignatureBuilder::from_base64(&file, URL_SAFE_NO_PAD, &input).unwrap()
            .build().unwrap(),
        input.into_inner()
    );

    println!("{:?}", serve.get_signature() );

    (Status::Ok, serde_json::json!("Subscribed!").to_string())
}

#[get("/send-notification")]
async fn send_notification(serve: &State<NotificationServer>) -> Status {
    serve.notification("Your mouth is dry.");

    let s = serve.get_sub();
    let mut builder = WebPushMessageBuilder::new(&s);
    let n = serve.get_notification();
    let content = n.as_bytes();
    builder.set_payload(ContentEncoding::Aes128Gcm, content);
    let g = serve.get_signature().clone();
    builder.set_vapid_signature(g);

    let message = builder.build().unwrap();
    let client = IsahcWebPushClient::new().unwrap();
    client.send(message).await.unwrap();
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, backend_msg, files, save_subscription, send_notification])
        .manage(NotificationServer::new())
        .attach(user::stage())
        .attach(ap::stage())
}
