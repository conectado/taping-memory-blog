#![recursion_limit = "256"]
use root::Root;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod blog_displayer;
mod blog_preview_list;
mod request_loader;
mod root;
mod spinner;

#[wasm_bindgen(start)]
pub async fn run_app() {
    App::<Root>::new().mount_as_body();
}
