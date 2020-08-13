#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;

#[get("/hello")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount(
            "/",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .launch();
}
