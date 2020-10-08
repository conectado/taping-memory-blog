use http::{Request, Response};
use yew::format::Text;
use yew::prelude::*;
use yew::services::{fetch::FetchTask, FetchService};
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader<
    T: Displayer<U, V> + 'static,
    U: From<Text> + 'static,
    V: Clone + Default + PartialEq + 'static,
> {
    props: RequestLoaderProps<V>,
    fetch_task: FetchTask,
    display_value: Option<U>,
    link: ComponentLink<Self>,
}

pub trait Displayer<U, T> {
    fn display(value: &Option<U>, extra_args: T) -> Html;
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps<T: Clone + Default + PartialEq> {
    pub url: String,
    #[prop_or_default]
    pub extra_args: T,
}

pub enum FetchMessage<T> {
    Loaded(T),
}

impl<
        T: Displayer<U, V> + 'static,
        U: From<Text> + 'static,
        V: Clone + Default + PartialEq + 'static,
    > Component for RequestLoader<T, U, V>
{
    type Properties = RequestLoaderProps<V>;
    type Message = FetchMessage<U>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_task = fetch_link(&props.url, &link);
        RequestLoader {
            props,
            fetch_task,
            display_value: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let FetchMessage::Loaded(value) = msg;
        self.display_value = Some(value);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.display_value = None;
            self.props = props;
            self.fetch_task = fetch_link(&self.props.url, &self.link);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            {
                T::display(&self.display_value, self.props.extra_args.clone())
            }
        }
    }
}

fn fetch_link<T: Displayer<U, V>, U: From<Text>, V: Default + PartialEq + 'static + Clone>(
    url: &str,
    link: &ComponentLink<RequestLoader<T, U, V>>,
) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();

    let callback =
        link.callback(|response: Response<U>| FetchMessage::Loaded(response.into_body()));

    FetchService::fetch(get_req, callback).unwrap()
}
