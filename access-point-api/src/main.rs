mod ap;
mod report;
mod user;

#[macro_use] extern crate rocket;

use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;

const FRONTEND_LOCATION: &str = "../access-point-ui/dist/";

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new(FRONTEND_LOCATION).join("index.html")).await.ok()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new(FRONTEND_LOCATION).join(file)).await.ok()
}

#[get("/backend-msg")]
async fn backend_msg() -> String {
    "Rocket backend!".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, backend_msg, files])
        .attach(user::stage())
}
