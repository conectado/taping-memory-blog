use yew::html;
use yew::virtual_dom::VNode;

pub fn spinner() -> VNode {
    html! {
        <div class="spinner-border" role="status">
            <span class="sr-only">{"Loading..."}</span>
        </div>
    }
}
