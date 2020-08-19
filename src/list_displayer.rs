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
                                        <RouterAnchor<AppRoute> route={AppRoute::ViewPost(item.clone())}>{item}</RouterAnchor<AppRoute>>
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
