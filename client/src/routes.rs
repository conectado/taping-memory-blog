use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/#about_me"]
    AboutMe,
    #[to = "/#articles/{post_name}"]
    ViewPost(String),
    #[to = "/#page={page_number}"]
    Page(usize),
    #[to = "/"]
    HomePage,
}
