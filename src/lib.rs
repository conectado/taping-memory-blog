mod blog_post;
pub use blog_post::model::{BlogEntry, Props};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub async fn run_app() {
    let text = "# test".to_string();
    App::<BlogEntry>::new().mount_as_body_with_props(Props { data: text });
}
