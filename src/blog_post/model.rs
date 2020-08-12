use super::view_code;

use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct BlogEntry {
    pub props: Props,
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    pub data: String,
}

impl Component for BlogEntry {
    type Message = ();
    type Properties = Props;

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        BlogEntry { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let mut subnav_class = "sub theme-black border-white center ".to_string();
        subnav_class.push_str("inherit-display");

        html! {
            <section>
                { self.view_data() }
            </section>
        }
    }
}

impl BlogEntry {
    pub fn view_data(&self) -> Html {
        html! {
            <section>
                {view_code(&self.props.data)}
            </section>
        }
    }
}
