#![recursion_limit = "256"]
use stdweb::js;
use stdweb::unstable::TryFrom;
use stdweb::web::Node;
use yew::prelude::*;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};

use yew::events::IKeyboardEvent;
use yew::format::{Nothing, Text};
use yew::virtual_dom::VNode;
use yew::{html, Component, ComponentLink, Html, Renderable, ShouldRender};

pub fn view_code(value: &str) -> Html<Model> {
    let markdown = js! {
        console.log("Document loaded");
        const div = document.createElement("div");
        div.className = "markdown";
        const code = @{&value};

        const options = {
            gfm: true,
            breaks: false,
            smartLists: true,
            smartypants: false,
            langPrefix: "hljs",
            highlight: (code, lang) => {
                if(!!(lang && hljs.getLanguage(lang))) {
                    return hljs.highlight(lang, code).value;
                }

                return code;
            }
        };

        marked.setOptions(options);

        div.innerHTML = marked(code);

        return div;
    };

    eprintln!("markdown: {:?}", markdown);
    let node = Node::try_from(markdown).expect("convert markdown");
    let vnode = VNode::VRef(node);
    eprintln!("div: {:?}", vnode);

    html! {
        { vnode }
    }
}

pub struct Model {
    console: ConsoleService,
    data: Option<String>,
    edit_value: String,
    fetch_service: FetchService,
    fetching: bool,
    ft: Option<FetchTask>,
    link: ComponentLink<Model>,
    value: String,
}

#[derive(Debug)]
pub enum Msg {
    Update(String),
    Submit,
    FetchReady(Text),
    Ignore,
    Nope,
}

impl Model {
    fn view_data(&self) -> Html<Model> {
        if let Some(value) = &self.data {
            html! {
                <section>
                    //<p>{"Loaded!"}</p>
                    {view_code(&value)}
                </section>
            }
        } else {
            html! {
                {"Loading..."}
            }
        }
    }

    fn fetch_data(&mut self) {
        self.fetching = true;

        let callback = self.link.send_back(move |response: Response<Text>| {
            let (meta, data) = response.into_parts();

            if meta.status.is_success() {
                Msg::FetchReady(data)
            } else {
                Msg::Ignore
            }
        });

        let request = Request::builder()
            .method("GET")
            .uri(self.value.clone())
            .body(Nothing)
            .unwrap();
        let task = self.fetch_service.fetch(request, callback);
        self.ft = Some(task);
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            fetch_service: FetchService::new(),
            console: ConsoleService::new(),
            fetching: false,
            edit_value: "".to_string(),
            value:
                "https://raw.githubusercontent.com/steadylearner/Rust-Full-Stack/master/README.md"
                    .to_string(),
            link,
            data: None,
            ft: None,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        self.fetch_data();

        true
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.edit_value = val;
            }
            Msg::Submit => {
                self.value = self.edit_value.clone();
                self.fetch_data();
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok();
            }
            Msg::Ignore => {
                return false;
            }
            Msg::Nope => {}
        }
        true
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        let mut subnav_class = "sub theme-black border-white center ".to_string();

        if self.fetching {
            subnav_class.push_str("x-display");
        } else {
            subnav_class.push_str("inherit-display");
        }

        html! {
            <section>
                <input
                    type="text",
                    autocomplete="off",
                    disabled=self.fetching,
                    value=&self.value,
                    oninput=|e| Msg::Update(e.value),
                    onkeypress=|e| {
                        if e.key() == "Enter" { Msg::Submit } else { Msg::Nope }
                    },
                />
                { self.view_data() }
            </section>
        }
    }
}
