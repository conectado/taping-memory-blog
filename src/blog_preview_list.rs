use yew::virtual_dom::VNode;

use crate::article_list::Articles;
use crate::blog_displayer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::routes::AppRoute;
use crate::spinner::spinner;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub type BlogPreviewListDisplayerComponent =
    RequestLoader<BlogPreviewListDisplayer, Json<Result<Articles, Error>>>;

pub struct BlogPreviewListDisplayer;

impl Displayer<Json<Result<Articles, Error>>> for BlogPreviewListDisplayer {
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> VNode {
        match text {
            Some(json) => match &json.0 {
                Ok(arts) => {
                    html! {
                        {
                            for arts.articles.iter().map(|item| {
                                html!{
                                    <div class="container rounded bg-element-dark" style="margin-top: 1%; display: -webkit-box; -webkit-box-orient: vertical;">
                                        <div style="-webkit-line-clamp: 8; overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical;">
                                            <BlogDisplayerComponent  url={("/preview/articles/".to_string() + item)} />
                                        </div>
                                        <div class="text-right" style="display: block; margin: 1em; font-size: 1.1em;">
                                            <RouterAnchor<AppRoute>  route={AppRoute::ViewPost(item.clone())}>{"See more..."}</RouterAnchor<AppRoute>>
                                        </div>
                                    </div>
                                }
                            })
                        }
                    }
                }
                _ => html! {<p>{"Error"}</p>},
            },
            None => spinner(),
        }
    }
}
