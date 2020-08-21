use crate::blog_displayer::BlogDisplayerComponent;
use crate::blog_preview_list::BlogPreviewListDisplayerComponent;
use crate::constants;
use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

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
                <div class="text-white" style="overflow: auto; position: fixed; height: 100%; width: 100%; background-color: black;">
                    <div class="bg-dark">
                        <div class="container" style="display: inline">
                            <RouterAnchor<AppRoute> route={AppRoute::List}>{"Home"}</RouterAnchor<AppRoute>>
                        </div>
                        <h3 class="text-center font-weight-bold container" style="padding-top: 0.5em; padding-bottom: 0.5em; display: inline flow-root;">
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
