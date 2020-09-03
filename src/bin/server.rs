#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::ContentType;
use rocket::response::NamedFile;
use rocket::{get, routes};
use rocket_contrib::{json::Json, serve::StaticFiles};

use std::fs;
use std::path::{Path, PathBuf};

use web_blog_lib::{
    article_list::Articles,
    constants,
    encoded::{AcceptEncodingHeader, EncodedContent},
};

#[get("/article_list")]
fn list_articles() -> Json<Articles> {
    let articles_path = format!("{}/{}", constants::STATIC_URL, constants::ARTICLES_PATH);
    let mut articles: Vec<_> = fs::read_dir(&articles_path)
        .unwrap_or_else(|_| {
            panic!(
                "Error ocurred while listing statics files in directory: {}",
                &articles_path
            )
        })
        .collect();

    articles.sort_by(|a, b| {
        a.as_ref()
            .unwrap()
            .file_name()
            .cmp(&b.as_ref().unwrap().file_name())
    });

    articles.reverse();

    let articles = articles
        .iter()
        .map(|res| res.as_ref().unwrap().file_name().into_string().unwrap())
        .collect();

    Json(Articles { articles })
}

#[get("/<path..>")]
fn get_article(path: PathBuf, encoding: AcceptEncodingHeader) -> Option<EncodedContent> {
    let file =
        NamedFile::open(Path::new(constants::STATIC_URL).join(path)).expect("File not found");
    let content_type =
        ContentType::from_extension(file.path().extension().unwrap().to_str().unwrap());
    Some(EncodedContent::new(
        file,
        encoding.accept_encoding,
        content_type.unwrap_or(ContentType::Plain),
    ))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![list_articles])
        .mount("/", routes![get_article])
        .mount("/", StaticFiles::from(constants::STATIC_URL))
        .launch();
}
