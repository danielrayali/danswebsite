#[macro_use]
extern crate rocket;
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::stream::ReaderStream;
use rocket::tokio::fs::File;
use std::path::Path;

#[get("/<name>")]
async fn read_section(name: &str) -> std::io::Result<(ContentType, ReaderStream![File])> {
    let file_to_serve = name.to_string() + ".html";
    let file = File::open(file_to_serve).await?;
    Ok((ContentType::HTML, ReaderStream::one(file)))
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("blog.html")).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![read_section])
}
