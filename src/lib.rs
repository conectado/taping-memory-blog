mod blog_displayer;
mod request_loader;
mod root;
pub use request_loader::{RequestLoader, RequestLoaderProps};
use root::{Root, RootProperties};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub async fn run_app() {
    App::<Root>::new().mount_as_body_with_props(RootProperties {
        url: "/articles/test.md".to_string(),
    });
}
