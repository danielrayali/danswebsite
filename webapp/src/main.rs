#[macro_use]
extern crate rocket;
use rocket::http::{ContentType, Status};
use rocket::tokio::fs;
use rocket::Request;
use rocket_dyn_templates::{context, Template};
use std::path::Path;

async fn get_blog() -> Template {
    Template::render(
        "blog",
        context! {
            dansstyle: fs::read_to_string("styles.css").await.unwrap()
        },
    )
}

#[get("/")]
async fn index() -> Template {
    get_blog().await
}

#[get("/blog")]
async fn blog() -> Template {
    get_blog().await
}

#[get("/about")]
async fn about() -> Template {
    Template::render(
        "about",
        context! {
            dansstyle: fs::read_to_string("styles.css").await.unwrap()
        },
    )
}

#[get("/<name>")]
async fn read_path(name: &str) -> (Status, Option<(ContentType, std::io::Result<String>)>) {
    let file_path_str = name.to_string();
    if !file_path_str.ends_with(".html") && !file_path_str.ends_with(".css") {
        return (Status::NotFound, None);
    }
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

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no! We couldn't find the requested path '{}'", req.uri())
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .register("/", catchers![not_found])
        .attach(Template::fairing())
        .mount("/", routes![index, blog, read_path, about])
        .launch()
        .await?;

    Ok(())
}
