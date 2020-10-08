use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::spinner::spinner;
use anyhow::Error;
use pulldown_cmark as pc;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::virtual_dom::VNode;
use yew::{web_sys, Html};

#[wasm_bindgen]
extern "C" {
    type hljs;

    #[wasm_bindgen(static_method_of = hljs)]
    pub fn highlightBlock(block: JsValue);
}

pub type BlogDisplayerComponent = RequestLoader<BlogDisplayer, Result<String, Error>, ()>;

fn create_markdown_container() -> web_sys::Element {
    let window = web_sys::window().expect("Can't find window");
    let document = window.document().expect("Can't find document");
    let div = document.create_element("div").expect("Couldn't create div");
    div.set_class_name("markdown-body");
    div
}

fn view_markdown(value: &str) -> Html {
    let parser = pc::Parser::new(value);
    let mut html_output = String::new();
    pc::html::push_html(&mut html_output, parser);

    let div = create_markdown_container();

    div.set_inner_html(&html_output);

    let code_blocks = div.query_selector_all("pre code").unwrap();
    for i in 0..code_blocks.length() {
        hljs::highlightBlock(JsValue::from(code_blocks.get(i).unwrap()));
    }

    let node = web_sys::Node::from(div);
    VNode::VRef(node)
}

pub struct BlogDisplayer;

impl Displayer<Result<String, Error>, ()> for BlogDisplayer {
    fn display(text: &Option<Result<String, Error>>, _: ()) -> VNode {
        html! {
            <div style="word-break: break-word" class="bg-element-dark">
                {
                    match &text {
                        Some(result) => match result {
                            Ok(value) => html! {
                                <div style="word-break: break-word" class="text-element-white markdown-body">
                                    {view_markdown(value)}
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
