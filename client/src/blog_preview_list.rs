use crate::blog_displayer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::routes::AppRoute;
use crate::spinner::spinner;
use anyhow::Error;
use shared::article_list::Articles;
use std::cmp::min;
use yew::format::Json;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

const PAGE_SIZE: usize = 5;

pub type BlogPreviewListDisplayerComponent =
    RequestLoader<BlogPreviewListDisplayer, Json<Result<Articles, Error>>, usize>;

pub struct BlogPreviewListDisplayer;

impl Displayer<Json<Result<Articles, Error>>, usize> for BlogPreviewListDisplayer {
    fn display(value: &Option<Json<Result<Articles, Error>>>, page_number: usize) -> Html {
        let (start_index, end_index) = calculate_indexes_from_page(page_number);
        match value {
            Some(json) => match &json.0 {
                Ok(arts) => {
                    html! {
                        <>
                            <div class="row">
                                {
                                    for arts.articles[start_index..min(arts.articles.len(), end_index)].iter().map(|article| {
                                        display_article(article)
                                    })

                                }
                            </div>
                            <div class="row">
                                {
                                    if page_number > 1 {
                                        html! {
                                            <RouterAnchor<AppRoute> classes="col-1 pageButton bg-element-dark" route={AppRoute::Page(page_number - 1)}>
                                                <div class="pageButton">
                                                    {"<<"}
                                                </div>
                                            </RouterAnchor<AppRoute>>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                                {
                                    if end_index < arts.articles.len() {
                                        html! {
                                            <RouterAnchor<AppRoute> classes="col-1 offset-10 pageButton bg-element-dark"  route={AppRoute::Page(page_number + 1)}>
                                                <div class="pageButton">
                                                    {">>"}
                                                </div>
                                            </RouterAnchor<AppRoute>>
                                        }
                                    } else {
                                        html! {}
                                    }
                                }
                            </div>
                        </>
                    }
                }
                _ => html! {<p>{"Error"}</p>},
            },
            None => spinner(),
        }
    }
}

fn display_article(article: &str) -> Html {
    html! {
        <div class="container rounded previewer" style="margin-top: 1%; display: -webkit-box; -webkit-box-orient: vertical;">
            <RouterAnchor<AppRoute>  route={AppRoute::ViewPost(article.to_string())}>
                <div style="-webkit-line-clamp: 8; overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical;">
                    <BlogDisplayerComponent  url={("/preview/articles/".to_string() + article)} />
                </div>
            </RouterAnchor<AppRoute>>
        </div>
    }
}

fn calculate_indexes_from_page(page_number: usize) -> (usize, usize) {
    let start_index = (page_number - 1) * PAGE_SIZE;
    let end_index = start_index + PAGE_SIZE;
    (start_index, end_index)
}
