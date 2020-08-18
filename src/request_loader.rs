use http::{Request, Response};
use yew::format::Text;
use yew::prelude::*;
use yew::services::{fetch::FetchTask, FetchService};
use yew::virtual_dom::VNode;
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader<T: Displayer<U> + 'static, U: From<Text> + 'static> {
    props: RequestLoaderProps,
    phantom: std::marker::PhantomData<T>,
    #[allow(dead_code)]
    fetch_task: FetchTask, // Needed to keep the ref alive in scope
    display_text: Option<U>,
}

pub trait Displayer<U> {
    fn display(text: &Option<U>) -> VNode;
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps {
    pub url: String,
}

pub enum FetchMessage<T> {
    Success(T),
    Failure,
}

impl<T: Displayer<U> + 'static, U: From<Text> + 'static> Component for RequestLoader<T, U> {
    type Properties = RequestLoaderProps;
    type Message = FetchMessage<U>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_task = fetch_article_list(&props.url, &link);
        RequestLoader {
            props,
            phantom: std::marker::PhantomData,
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
                self.display_text = None;
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

fn fetch_article_list<T: Displayer<U>, U: From<Text>>(
    url: &str,
    link: &ComponentLink<RequestLoader<T, U>>,
) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback = link.callback(|response: Response<U>| {
        if response.status().is_success() {
            FetchMessage::Success(response.into_body())
        } else {
            FetchMessage::Failure
        }
    });

    FetchService::fetch(get_req, callback).unwrap()
}
