use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::spinner::spinner;
use anyhow::Error;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::virtual_dom::VNode;
use yew::{web_sys, Html};

pub type BlogDisplayerComponent = RequestLoader<BlogDisplayer, Result<String, Error>>;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Options {
    pub gfm: bool,
    pub breaks: bool,
    pub header_ids: bool,
    pub smart_lists: bool,
    pub smarty_pants: bool,
}

#[wasm_bindgen(module = "/js_snippets/set_marked_options.js")]
extern "C" {
    fn set_marked_options(info: JsValue);
}

#[wasm_bindgen]
extern "C" {
    fn marked(code: &str) -> String;
}

fn create_markdown_container() -> web_sys::Element {
    let window = web_sys::window().expect("Can't find window");
    let document = window.document().expect("Can't find document");
    let div = document.create_element("div").expect("Couldn't create div");
    div.set_class_name("markdown");
    div
}

fn view_code(value: &str) -> Html {
    let div = create_markdown_container();

    let options = JsValue::from_serde(&Options {
        gfm: true,
        breaks: false,
        header_ids: true,
        smart_lists: true,
        smarty_pants: true,
    })
    .unwrap();

    set_marked_options(options);

    div.set_inner_html(marked(value).as_ref());

    let node = web_sys::Node::from(div);
    VNode::VRef(node)
}

pub struct BlogDisplayer;

impl Displayer<Result<String, Error>> for BlogDisplayer {
    fn display(text: &Option<Result<String, Error>>) -> VNode {
        html! {
            <div style="padding: 1em; word-break: break-word" class="container bg-dark">
                {
                    match &text {
                        Some(result) => match result {
                            Ok(value) => html! {
                                <div style="padding: 1em; word-break: break-word" class="text-white container markdown-body">
                                    {view_code(value)}
                                </div>
                            },
                            _ => html! { <p>{"error"}</p> },
                        },
                        None => spinner(),
                    }
                }
            </div>
        }
    }
}