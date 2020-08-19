use crate::article_list::Articles;
use crate::blog_displayer::BlogDisplayer;
use crate::constants;
use crate::list_displayer::ListDisplayer;
use crate::request_loader::RequestLoader;
use anyhow::Error;
use yew::format::Json;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/articles/{post_name}"]
    ViewPost(String),
    #[to = "/"]
    List,
}

pub struct Root {}

impl Component for Root {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute|
                    match switch {
                        AppRoute::ViewPost(article) => html! {
                            <RequestLoader<BlogDisplayer, Result<String, Error>> url={("/articles/".to_string() + &article[..])}/>
                        },
                        AppRoute::List => html! {
                            <RequestLoader<ListDisplayer, Json<Result<Articles, Error>>> url=constants::ARTICLE_LIST_URI/>
                        },
                    }
                )
            />
        }
    }
}
