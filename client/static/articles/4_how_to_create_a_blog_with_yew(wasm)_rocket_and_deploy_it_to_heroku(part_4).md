# How I built this blog using only Rust (part 4)
## Highlighting the code

#### Adding a root component

To make our life easier let's create a root component in `src/root.rs`:

```
src/root.rs
---
use crate::constants;
use crate::markdown_preview_list::BlogPreviewListDisplayerComponent;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Root;

impl Component for Root {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="text-white" style="overflow: auto; position: fixed; height: 100%; width: 100%; background-color: black;">
                        <BlogPreviewListDisplayerComponent url=constants::ARTICLE_LIST_URI/>
                </div>
            </body>
        }
    }
}
```

This root simply encapsulates our `BlogPreviewListDisplayerComponent` withing a div and a body for easy usage.

Let's use this component in `src/lib.rs`:

```
src/lib.rs
---
#![recursion_limit = "256"]

use root::Root;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod markdown_preview_list;
mod markdown_visualizer;
mod request_loader;
mod root;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_as_body();
}
```

The root component use some styles so let's add them in `static/index.html`:

```
static/index.html
---
<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="blog">
    <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" integrity="sha384-JcKb8q3iqJ61gNV9KGb8thSsNjpSL0n8PARn9HuZOnIxN0hoP+VmmDGMN5t9UJ0Z" crossorigin="anonymous">
    <script src="https://kit.fontawesome.com/15c3238942.js" crossorigin="anonymous"></script>
    <title>blog</title>
    <script type="module">
      import init from "./build/wasm.js";
      init();
    </script>
  </head>
</html>
```


Compile and run:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build --release
cargo run
```

Head to http://localhost:8000/ and you will see a slightly prettier version of the last stage.

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-8)

#### Adding routing

Since this is a SPA we will add routing.

Routing, simply is a way to indicate the APP using the URL which component should be shown, making it work smoothly with the backward and forward buttons of the browser and making it easy redirecting to other components in the webpage. Luckily, there is already a library for Yew that does this for us.

Add `yew-router="^0.14"` to `Cargo.toml`:

```
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["<Your username>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "lib"]
name = "blog"

[dependencies]
yew = "^0.17"
wasm-bindgen = "^0.2"
pulldown-cmark = "^0.7"
http = "^0.2"
anyhow = "^1.0"
rocket = "^0.4"
serde = "^1.0"
serde_derive = "^1.0"
yew-router = "^0.14"

[dependencies.rocket_contrib]
version = "^0.4"
features = ["serve"]

[dependencies.web-sys]
version = "*"
features = ["NodeList"]

[[bin]]
name = "server"

```

Add a `src/routes.rs` file that will hold an enum with all the available routes in the application:

```rs
src/routes.rs
---
use yew_router::prelude::*;

#[derive(Switch, Clone)]
pub enum AppRoute {
    #[to = "/#articles/{post_name}"]
    ViewPost(String),
    #[to = "/"]
    List,
}
```

Notice the `to` attribute which relates an URL with a value of the enum. Moreover, it can grab part of the URL and match it with a type and put it inside a value of our enum. Meaning that when an URL follows the pattern `/#articles/{post_name}` the router will match the URL and return a `ViewPost(value)` where value is a `String`.

It is quite similar(we could say equivalent) to how a router in the back-end functions, it's just routing in the front-end.

Now, let's add the Router component to the `Root` element, so that it displays the articles previews or the article itself depending on the current URL:

```
src/root.rs
---
use crate::constants;
use crate::markdown_preview_list::BlogPreviewListDisplayerComponent;
use crate::markdown_visualizer::BlogDisplayerComponent;
use crate::routes::AppRoute;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew_router::components::RouterAnchor;
use yew_router::prelude::*;

pub struct Root;

impl Component for Root {
    type Properties = ();
    type Message = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <body>
                <div class="text-white" style="overflow: auto; position: fixed; height: 100%; width: 100%; background-color: black;">
                    <div class="bg-dark sticky-top">
                        <div class="container" style="display: inline">
                            <RouterAnchor<AppRoute> route={AppRoute::List}>
                                <i class="fas fa-home" style="font-size: 2em; color: white;"></i>
                            </RouterAnchor<AppRoute>>
                        </div>
                        <h3 class="text-center font-weight-bold container" style="padding-top: 0.5em; padding-bottom: 0.5em; display: inline flow-root;">
                            {"Blog"}
                        </h3>
                    </div>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute|
                            match switch {
                                AppRoute::ViewPost(article) => html! {
                                    <BlogDisplayerComponent url={format!("{}/{}", constants::ARTICLES_PATH,  &article[..])}/>
                                },
                                AppRoute::List => html! {
                                    <BlogPreviewListDisplayerComponent url=constants::ARTICLE_LIST_URI/>
                                },
                            }
                        )
                    />
                </div>
            </body>
        }
    }
}
```
Well, a slight improvement on styles were added but more importantly we added 2 things. First a `RouterAnchor`, this is just like an `a` element which instead of `href` as property has `route` which can use a Routing enum element which we point to `AppRoute::List` representing the home(related to the `/` URL).

Basically, it is just like a link to the homepage(if we ever change the URL in the enum the change would be reflected here). [More on the RouterAnchor](https://docs.rs/yew-router/0.14.0/yew_router/components/struct.RouterAnchor.html).

The other thing we added is the `Router` element, to understand more about how it works see [the documentation](https://docs.rs/yew-router/0.14.0/yew_router/router/struct.Router.html) and this [guide](https://yew.rs/docs/en/concepts/router/), but basically it is an element with a property `render` which takes a closure(wrapped by a `Router::render`) that has as an argument an `AppRoute` that simply represent the current state of the `AppRoute` based on the `URL` and it should return the component you want to render based on this. 

If the URL currently points to `AppRoute::List` it simply displays the preview list, and for anything matching `AppRoute::ViewPost(article)`(meaning an URL pattern akin to `/#articles/{article}` ) it will display the blog component with the URL corresponding to that article.


Finally, we also update the preview list to have links to the articles they preview:

```rs
src/markdown_preview_list.rs
---
use crate::article_list::Articles;
use crate::markdown_visualizer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::routes::AppRoute;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub type BlogPreviewListDisplayerComponent =
    RequestLoader<BlogPreviewListDisplayer, Json<Result<Articles, Error>>>;

pub struct BlogPreviewListDisplayer;

impl Displayer<Json<Result<Articles, Error>>> for BlogPreviewListDisplayer {
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> Html {
        match text {
            Some(json) => match &json.0 {
                Ok(arts) => {
                    html! {
                        {
                            for arts.articles.iter().map(|item| {
                                html!{
                                    <div class="container rounded bg-dark" style="margin-top: 1%; -webkit-line-clamp: 9; overflow: hidden; display: -webkit-box; -webkit-box-orient: vertical;">
                                        <div style="display: block; overflow: hidden; ">
                                            <BlogDisplayerComponent  url={("/articles/".to_string() + item)} />
                                        </div>
                                        <div class="text-right" style="display: block; margin: 1em; font-size: 1.1em;">
                                            <RouterAnchor<AppRoute>  route={AppRoute::ViewPost(item.clone())}>{"See more..."}</RouterAnchor<AppRoute>>
                                        </div>
                                    </div>
                                }
                            })
                        }
                    }
                }
                _ => html! {<p>{"Error"}</p>},
            },
            None => html! {<p>{"Loading.."}</p>},
        }
    }
}
```

And with this you can test your now navigable blog:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build --release
cargo run
```

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-9)

#### Adding syntax highlighting

As the last thing let's add syntax highlighting for the markdown visualizer

For highlighting I only found a JS library called `hljs`. So we have to deal wiith the boundary between JS and Rust.

In `src/markdown_visualizer.rs` let's deal with this.

```
src/markdown_visualizer.rs
---
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use anyhow::Error;
use pulldown_cmark as pc;
use wasm_bindgen::prelude::*;
use yew::html;
use yew::virtual_dom::VNode;
use yew::{web_sys, Html};

#[wasm_bindgen]
extern "C" {
    type hljs;

    #[wasm_bindgen(static_method_of = hljs)]
    pub fn highlightBlock(block: JsValue);
}

pub type BlogDisplayerComponent = RequestLoader<BlogDisplayer, Result<String, Error>>;

fn create_markdown_container() -> web_sys::Element {
    let window = web_sys::window().expect("Can't find window");
    let document = window.document().expect("Can't find document");
    let div = document.create_element("div").expect("Couldn't create div");
    div.set_class_name("markdown-body");
    div
}

fn view_markdown(value: &str) -> Html {
    let parser = pc::Parser::new(value);
    let mut html_output = String::new();
    pc::html::push_html(&mut html_output, parser);

    let div = create_markdown_container();

    div.set_inner_html(&html_output);
    let code_blocks = div.query_selector_all("pre code").unwrap();
    for i in 0..code_blocks.length() {
        hljs::highlightBlock(JsValue::from(code_blocks.get(i).unwrap()));
    }

    let node = web_sys::Node::from(div);
    VNode::VRef(node)
}

pub struct BlogDisplayer;

impl Displayer<Result<String, Error>> for BlogDisplayer {
    fn display(text: &Option<Result<String, Error>>) -> VNode {
        html! {
            <div style="padding: 1em; word-break: break-word" class="container bg-dark">
                {
                    match &text {
                        Some(result) => match result {
                            Ok(value) => html! {
                                <div style="padding: 1em; word-break: break-word" class="text-white container markdown-body">
                                    {view_markdown(value)}
                                </div>
                            },
                            _ => html! { <p>{"error"}</p> },
                        },
                        None => html! {{"Loading..."}},
                    }
                }
            </div>
        }
    }
}
```

The first thing we notice here is:

```rs
#[wasm_bindgen]
extern "C" {
    type hljs;

    #[wasm_bindgen(static_method_of = hljs)]
    pub fn highlightBlock(block: JsValue);
}
```

`extern "C"` means, this is an external function, tells the compiler "you will not find the definition here"(among other things). Then the `#[wasm_bindgen]` attribute tells WASM bindgen to do its magic and create the bindings with JS.

Then, we analyze the new block in the `view_markdown` code:

```rs
let code_blocks = div.query_selector_all("pre code").unwrap();
for i in 0..code_blocks.length() {
    hljs::highlightBlock(JsValue::from(code_blocks.get(i).unwrap()));
}
```

Well, this is part of the Browser's API, which we have access to thanks to `web_sys`. `query_selector_all("pre code")` give us all the child nodes of the div and allows us to highlight that node when passing it to `hljs`. More on this API [here](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll).

To use this API we need to enable the feature `NodeList` in `web_sys`, let's do it in the `Cargo.toml`:

```rs
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["<Your username>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib", "lib"]
name = "blog"

[dependencies]
yew = "^0.17"
wasm-bindgen = "^0.2"
pulldown-cmark = "^0.7"
http = "^0.2"
anyhow = "^1.0"
rocket = "^0.4"
serde = "^1.0"
serde_derive = "^1.0"
yew-router = "^0.14"

[dependencies.rocket_contrib]
version = "^0.4"
features = ["serve"]

[dependencies.web-sys]
version = "*"
features = ["NodeList"]

[[bin]]
name = "server"
```

Finally, we need to add the hljs lib to the HTML:

```rs
static/index.html
---
<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="blog">
    <link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/css/bootstrap.min.css" integrity="sha384-JcKb8q3iqJ61gNV9KGb8thSsNjpSL0n8PARn9HuZOnIxN0hoP+VmmDGMN5t9UJ0Z" crossorigin="anonymous">
    <script src="https://kit.fontawesome.com/15c3238942.js" crossorigin="anonymous"></script>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/9.18.3/styles/solarized-dark.min.css" integrity="sha512-kfScFZlIKxzC815vfFWpliT1aOaN0tS2QJDGcmCQ87Cai75745cB57HbyERuZsQXmcF0TX5qgfDdVAW7QCOxQQ==" crossorigin="anonymous" />
    <script src="//cdnjs.cloudflare.com/ajax/libs/highlight.js/10.1.1/highlight.min.js"></script>
    <script charset="UTF-8"
            src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/10.0.0/languages/rust.min.js"></script>
    <title>blog</title>
    <script type="module">
      import init from "./build/wasm.js";
      init();
    </script>
  </head>
</html>
```

And now let's change the first article for something with a little rust code so we see how it's highlighted

```
static/articles/test.md
---

\```rs
fn main() {
  println!("hello world!");
}
\```
```

Notice that you need to erase the backslashes. The artifacts of writing literal markdown in markdown.


Compile and run:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build --release
cargo run
```

And tada! we are done, next blog we will see how to deploy this blog to heroku.

...

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-10)

## [Part 5](#articles/5_how_to_create_a_blog_with_yew(wasm)_rocket_and_deploy_it_to_heroku(part_5).md)
