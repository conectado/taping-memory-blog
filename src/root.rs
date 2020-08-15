use crate::blog_displayer::BlogDisplayer;
use crate::RequestLoader;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Root {
    props: RootProperties,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RootProperties {
    pub url: String,
}

impl Component for Root {
    type Properties = RootProperties;
    type Message = ();

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root { props }
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <RequestLoader<BlogDisplayer> url=&self.props.url/>
        }
    }
}
