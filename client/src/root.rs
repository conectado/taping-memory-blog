use crate::about_me::AboutMe;
use crate::blog_displayer::BlogDisplayerComponent;
use crate::blog_preview_list::BlogPreviewListDisplayerComponent;
use crate::routes::AppRoute;
use shared::constants;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

pub struct Root;

// TODO: Why is root being created twice?

impl Component for Root {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root
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
                {header()}
                {main_page()}
            </body>
        }
    }
}

fn display_page(page_number: usize) -> Html {
    html! {
        <BlogPreviewListDisplayerComponent url=constants::ARTICLE_LIST_URI extra_args = page_number/>

    }
}

fn main_page() -> Html {
    html! {
        <div class="container-flow" style="overflow-x: hidden; overflow-y: auto; position: relative; height: 100%; width: 100%;" id="background">
            <div class="row">
                <div class="col-6 offset-3">
                    <Router<AppRoute, ()> render = Router::render(routing) />
                </div>
            </div>
        </div>
    }
}

fn routing(switch: AppRoute) -> Html {
    match switch {
        AppRoute::ViewPost(article) => html! {
            <BlogDisplayerComponent url={format!("/{}/{}", constants::ARTICLES_PATH,  &article[..])}/>
        },
        AppRoute::Page(page_number) => display_page(page_number),
        AppRoute::AboutMe => html! {<AboutMe />},
        AppRoute::HomePage => display_page(1),
    }
}

fn header() -> Html {
    html! {
        <h3 class="font-weight-bold header sticky-top container-fluid">
            <div class="row align-items-end">
                    <RouterAnchor<AppRoute> route={AppRoute::HomePage} classes="col-3">
                            <img src="./imgs/kitten.png#logo" />
                    </RouterAnchor<AppRoute>>
                    <div class="col-1 offset-6">
                        <a href="https://ko-fi.com/S6S529BSG" target="_blank" style="position: absolute; bottom: 0;">
                            {"Buy me a coffee ☕"}
                        </a>
                    </div>
                    <div class="col-1">
                        <div style="position: absolute; bottom: 0;">
                            <RouterAnchor<AppRoute> route={AppRoute::AboutMe}>
                                {"About me"}
                            </RouterAnchor<AppRoute>>
                        </div>
                    </div>
            </div>
        </h3>
    }
}
