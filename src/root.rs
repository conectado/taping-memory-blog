use crate::blog_displayer::BlogDisplayerComponent;
use crate::blog_preview_list::BlogPreviewListDisplayerComponent;
use crate::constants;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/articles/{post_name}"]
    ViewPost(String),
    #[to = "/"]
    List,
}

pub struct Root;

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
            <body>
                <div class="bg-dark text-white" style="overflow: auto; position: fixed; height: 100%; width: 100%;">
                    <div>
                        <h3 class="container text-center font-weight-bold">
                            {"Conectado's Blog"}
                        </h3>
                    </div>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute|
                            match switch {
                                AppRoute::ViewPost(article) => html! {
                                    <BlogDisplayerComponent url={("/articles/".to_string() + &article[..])}/>
                                },
                                AppRoute::List => html! {
                                    <BlogPreviewListDisplayerComponent url=constants::ARTICLE_LIST_URI/>
                                },
                            }
                        )
                    />
                </div>
            </body>
        }
    }
}
