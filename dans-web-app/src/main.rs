#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::tokio::fs;
use std::path::Path;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("blog.html")).await.ok()
}

#[get("/<name>")]
async fn read_path(name: &str) -> (Status, Option<(ContentType, std::io::Result<String>)>) {
    let file_path_str = name.to_string() + ".html";
    let file_path = Path::new(&file_path_str);
    if !file_path.exists() {
        (Status::NotFound, None)
    } else {
        (
            Status::Ok,
            Some((ContentType::HTML, fs::read_to_string(file_path_str).await)),
        )
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![read_path])
}
