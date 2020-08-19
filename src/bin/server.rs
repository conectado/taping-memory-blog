#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{get, routes};
use rocket_contrib::{json::Json, serve::StaticFiles};

use std::fs;

use web_blog_lib::{article_list::Articles, constants};

#[get("/article_list")]
fn list_articles() -> Json<Articles> {
    let articles: Vec<String> = fs::read_dir(format!(
        "{}{}",
        constants::STATIC_URL,
        constants::ARTICLES_PATH
    ))
    .unwrap()
    .map(|res| res.unwrap().file_name().into_string().unwrap())
    .collect();

    Json(Articles { articles })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![list_articles])
        .mount("/", StaticFiles::from(constants::STATIC_URL))
        .launch();
}
