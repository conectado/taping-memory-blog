use crate::blog_displayer::BlogDisplayer;
use crate::list_displayer::Articles;
use crate::list_displayer::ListDisplayer;
use crate::RequestLoader;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;
use yew_router::service::RouteService;

const ARTICLES_URL: &'static str = "/article_list";
const ARTICLE_URL: &'static str = "/articles";

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/articles/{post_name}"]
    ViewPost(String),
    #[to = "/"]
    List,
}

pub struct Root {
    props: RootProperties,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RootProperties {
    pub url: String,
}

impl Component for Root {
    type Properties = RootProperties;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root { props }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let route_service = RouteService::<AppRoute>::new();

        html! {
            <Router<AppRoute, ()>
                render = Router::render(move |switch: AppRoute|
                    match switch {
                        AppRoute::ViewPost(_) => html! {
                            <RequestLoader<BlogDisplayer, Result<String, Error>> url=route_service.get_path()/>
                        },
                        AppRoute::List => html! {
                            <RequestLoader<ListDisplayer, Json<Result<Articles, Error>>> url=ARTICLES_URL/>
                        },
                    }
                )
            />
        }
        /*
        html! {
        }
        */
        /*
        html! {
            <RequestLoader<ListDisplayer, Json<Result<Articles, Error>>> url=&self.props.url/>
        }
        */
    }
}
