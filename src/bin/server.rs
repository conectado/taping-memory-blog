#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{get, response::content, routes};
use rocket_contrib::serve::StaticFiles;

use std::fs;

#[get("/article_list")]
fn list_articles() -> content::Json<String> {
    let articles: Vec<String> = fs::read_dir("./static/articles")
        .unwrap()
        .map(|res| res.unwrap().file_name().into_string().unwrap())
        .collect();

    content::Json(format!("{{\"articles\": {:?} }}", articles))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![list_articles])
        .mount("/", StaticFiles::from("./static"))
        .launch();
}
