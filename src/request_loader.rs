use anyhow::Error;
use http::{Request, Response};
use yew::prelude::*;
use yew::services::fetch::FetchTask;
use yew::services::FetchService;
use yew::virtual_dom::VNode;
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader<T: Displayer + 'static> {
    props: RequestLoaderProps,
    link: ComponentLink<Self>,
    #[allow(dead_code)]
    fetch_task: FetchTask, // Needed to keep the ref alive in scope
    display_text: Option<String>,
}

pub trait Displayer {
    fn display(text: &Option<String>) -> VNode;
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps {
    pub url: String,
}

pub enum FetchMessage {
    Success(String),
    Failure,
}

impl<T: Displayer + 'static> Component for RequestLoader<T> {
    type Properties = RequestLoaderProps;
    type Message = FetchMessage;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_task = fetch_article(&props.url, &link);
        RequestLoader {
            props,
            link,
            fetch_task,
            display_text: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            FetchMessage::Success(text) => {
                self.display_text = Some(text);
                true
            }
            FetchMessage::Failure => {
                self.display_text = Some("Failure :(".to_string());
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            {
                T::display(&self.display_text)
            }
        }
    }
}

fn fetch_article<T: Displayer>(url: &str, link: &ComponentLink<RequestLoader<T>>) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback = link.callback(|response: Response<Result<String, Error>>| {
        if response.status().is_success() {
            FetchMessage::Success(response.into_body().unwrap())
        } else {
            FetchMessage::Failure
        }
    });

    FetchService::fetch(get_req, callback).unwrap()
}
