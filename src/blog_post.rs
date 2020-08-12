use serde::Serialize;
use wasm_bindgen::prelude::*;
use yew::virtual_dom::VNode;
use yew::web_sys;
use yew::Html;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub gfm: bool,
    pub breaks: bool,
    pub header_ids: bool,
    pub smart_lists: bool,
    pub smarty_pants: bool,
}

#[wasm_bindgen(inline_js = "export function set_marked_options(info){ 
    console.log(info);
    info.highlight = (code, lang) => {
            if(!!(lang && hljs.getLanguage(lang))) 
            {
                return hljs.highlight(lang,code).value;
            } 

            return code;
        };

    marked.setOptions(info); 
    }")]
extern "C" {
    fn set_marked_options(info: JsValue);
}

#[wasm_bindgen]
extern "C" {
    fn marked(code: &str) -> String;
}

pub fn view_code(value: &str) -> Html {
    let window = web_sys::window().expect("Can't find window");
    let document = window.document().expect("Can't find document");
    let div = document.create_element("div").expect("Couldn't create div");
    div.set_class_name("markdown");
    let options = JsValue::from_serde(&Options {
        gfm: true,
        breaks: false,
        header_ids: false,
        smart_lists: true,
        smarty_pants: false,
    })
    .unwrap();

    set_marked_options(options);

    div.set_inner_html(marked(value).as_ref());
    let node = web_sys::Node::from(div);
    VNode::VRef(node)
}

pub mod model;
