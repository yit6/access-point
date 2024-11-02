#[macro_use] extern crate rocket;

use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(&dotenv::var("FRONTEND_LOCATION").expect("No frontend location found")).join("index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(&dotenv::var("FRONTEND_LOCATION").expect("No frontend location found")).join(file)).await.ok()
}

#[get("/backend-msg")]
async fn backend_msg() -> String {
    "Rocket backend!".to_string()
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    rocket::build().mount("/", routes![index, backend_msg, files])
}
