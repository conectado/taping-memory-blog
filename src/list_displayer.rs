use crate::article_list::Articles;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::root::AppRoute;
use crate::spinner::spinner;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::components::RouterAnchor;

pub type ListDisplayerComponent = RequestLoader<ListDisplayer, Json<Result<Articles, Error>>>;

pub struct ListDisplayer;

impl Displayer<Json<Result<Articles, Error>>> for ListDisplayer {
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> VNode {
        match text {
            Some(json) => match &json.0 {
                Ok(arts) => {
                    html! {
                        <ul>
                        {
                            for arts.articles.iter().map(|item| {
                                html!{
                                    <li>
                                        <RouterAnchor<AppRoute> route={AppRoute::ViewPost(item.clone())}>{convert_to_title(item)}</RouterAnchor<AppRoute>>
                                    </li>
                                }
                            })
                        }
                        </ul>
                    }
                }
                _ => html! {<p>{"Error"}</p>},
            },
            None => spinner(),
        }
    }
}

fn convert_to_title(file_name: &str) -> String {
    let title = file_name.replace("_", " ");
    let title: Vec<&str> = title.split(".").collect();
    let mut title = title[0].to_string();
    title.replace_range(0..1, &title[0..1].to_uppercase());
    title
}
