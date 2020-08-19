use yew::virtual_dom::VNode;

use crate::article_list::Articles;
use crate::blog_displayer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::spinner::spinner;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;

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
                                    <div class="border container rounded" style="margin-top: 1%">
                                        <BlogDisplayerComponent url={("/articles/".to_string() + item)} />
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
