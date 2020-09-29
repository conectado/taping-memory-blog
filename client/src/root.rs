use crate::about_me::AboutMe;
use crate::blog_displayer::BlogDisplayerComponent;
use crate::blog_preview_list::BlogPreviewListDisplayerComponent;
use crate::routes::AppRoute;
use shared::constants;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

pub struct Root;

impl Component for Root {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="text-element-white" style="overflow: auto; position: fixed; height: 100%; width: 100%;" id="background">
                    <div class="bg-element-dark sticky-top container-fluid">
                        <div class="row">
                            <div class="col-3" style="display: inline">
                                <RouterAnchor<AppRoute> route={AppRoute::List}>
                                    <i class="fas fa-home" style="font-size: 2em; color: white;"></i>
                                </RouterAnchor<AppRoute>>
                            </div>
                            <h3 class="font-weight-bold col-7" style="padding-top: 0.5em; padding-bottom: 0.5em; display: inline flow-root;">
                                {"Taping Memory"}
                            </h3>
                            <div class="col-1">
                                <a href="https://ko-fi.com/S6S529BSG" target="_blank">
                                    <img height="36" style="border:0px;height:36px;" src="https://cdn.ko-fi.com/cdn/kofi3.png?v=2" border="0" alt="Buy Me a Coffee at ko-fi.com" />
                                </a>
                            </div>
                            <div class="col-1">
                                <RouterAnchor<AppRoute> route={AppRoute::AboutMe}>
                                    {"About me"}
                                </RouterAnchor<AppRoute>>
                            </div>
                        </div>
                    </div>
                    <div class="container-flow">
                        <div class="row">
                            <div class="col-6 offset-3">
                                <Router<AppRoute, ()>
                                    render = Router::render(move |switch: AppRoute|
                                        match switch {
                                            AppRoute::ViewPost(article) => html! {
                                                <BlogDisplayerComponent url={format!("/{}/{}", constants::ARTICLES_PATH,  &article[..])}/>
                                            },
                                            AppRoute::List => html! {
                                                <BlogPreviewListDisplayerComponent url=constants::ARTICLE_LIST_URI/>
                                            },
                                            AppRoute::AboutMe => html! {
                                                <AboutMe />
                                            }
                                        }
                                    )
                                />
                            </div>
                        </div>
                    </div>
                </div>
            </body>
        }
    }
}
