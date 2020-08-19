use root::Root;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod blog_displayer;
mod list_displayer;
mod request_loader;
mod root;

#[wasm_bindgen(start)]
pub async fn run_app() {
    App::<Root>::new().mount_as_body();
}
