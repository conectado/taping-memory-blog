#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{get, response::content, routes};
use rocket_contrib::serve::StaticFiles;

use std::fs;

use web_blog_lib::constants;

#[get("/article_list")]
fn list_articles() -> content::Json<String> {
    let articles: Vec<String> = fs::read_dir(format!(
        "{}{}",
        constants::STATIC_URL,
        constants::ARTICLES_PATH
    ))
    .unwrap()
    .map(|res| res.unwrap().file_name().into_string().unwrap())
    .collect();

    content::Json(format!("{{\"articles\": {:?} }}", articles))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![list_articles])
        .mount("/", StaticFiles::from(constants::STATIC_URL))
        .launch();
}
