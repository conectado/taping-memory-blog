use crate::article_list::Articles;
use crate::request_loader::Displayer;
use crate::root::AppRoute;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew::virtual_dom::{VNode, VText};
use yew_router::components::RouterAnchor;

pub struct ListDisplayer {}

impl Displayer<Json<Result<Articles, Error>>> for ListDisplayer {
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> VNode {
        match text {
            Some(json) => match &json.0 {
                Ok(arts) => {
                    html! {{
                            for arts.articles.iter().map(|item| {
                                html!{
                                    <li>
                                        <RouterAnchor<AppRoute> route={AppRoute::ViewPost(item.clone())}>{convert_to_title(item)}</RouterAnchor<AppRoute>>
                                    </li>
                                }
                            })
                    }}
                }
                _ => VNode::from(VText::new("Error!".to_string())),
            },
            None => VNode::from(VText::new("Loading...".to_string())),
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
