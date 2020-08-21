use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/articles/{post_name}"]
    ViewPost(String),
    #[to = "/"]
    List,
}
