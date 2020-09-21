# How I built this blog using only Rust (part 3)
## Displaying a preview list of all the articles

### Preparing for the preview list
#### Adding a backend

Now, for adding a list of available articles we need a back-end(Well any back-end would do, you could use a github repo for example, but it has its battery of problems, that I will not address in this article).

So, let's add a back-end, we will use [Rocket](https://rocket.rs/), it's an easy to use server, which will do for our intentions.

First let add to the `Cargo.toml` a new `bin` as a target for compilation.

```toml
[[bin]]
name = "server"
```

Add `rocket = "^0.4"` to the dependencies.

And rocket_contrib to serve static files easily:

```rs
[dependencies.rocket_contrib]
version = "^0.4"
features = ["serve"]
```

And now `Cargo.toml` should look like

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
crate-type = ["cdylib"]

[dependencies]
yew = "^0.17"
wasm-bindgen = "^0.2"
pulldown-cmark = "^0.7"
http = "^0.2"
anyhow = "^1.0"
rocket = "^0.4"

[dependencies.rocket_contrib]
version = "^0.4"
features = ["serve"]

[[bin]]
name = "server"
```

And then let's add the server in `src/bin/server.rs`

```rs
src/bin/server.rs
---
#![feature(proc_macro_hygiene, decl_macro)]
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .mount("/", StaticFiles::from("./static"))
        .launch();
}
```

Now let's run the server an check that everything is working correctly

```bash
cargo run
```

[state-4](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-4)

#### Generalizing the RequestLoader component

Next step is generalizing the `RequestLoader` so that it knows how to handle different types of requests.

In this case we just want to show either an article or a list of articles. The way we prepared `request_loader` make it really easy to generalize it, we just need to make the `display` method of the `request_loader` change behaviour on whatever we want.

For that we will templatize the `display_value` to hold anything instead of just String, since the HTTP request are string-based we only ask that it can be casted to this type from `String`.(We ask its lifetime to be static because `Message` needs to be static)

Then we will also have a type that implements the trait `Displayer` that just describes a type that have an static method `display` that can be called to display something of type `U` the type of the `display_value`.

```
src/request_loader.rs
---
use http::{Request, Response};
use yew::format::Text;
use yew::prelude::*;
use yew::services::{fetch::FetchTask, FetchService};
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader<T: Displayer<U> + 'static, U: From<Text> + 'static> {
    props: RequestLoaderProps,
    fetch_task: FetchTask,
    display_value: Option<U>,
    link: ComponentLink<Self>,
}

pub trait Displayer<U> {
    fn display(value: &Option<U>) -> Html;
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps {
    pub url: String,
}

pub enum FetchMessage<T> {
    Loaded(T),
}

impl<T: Displayer<U> + 'static, U: From<Text> + 'static> Component for RequestLoader<T, U> {
    type Properties = RequestLoaderProps;
    type Message = FetchMessage<U>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_task = fetch_url(&props.url, &link);
        RequestLoader {
            props,
            fetch_task,
            display_value: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let FetchMessage::Loaded(value) = msg;
        self.display_value = Some(value);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.display_value = None;
            self.props = props;
            self.fetch_task = fetch_url(&self.props.url, &self.link);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            {
                T::display(&self.display_value)
            }
        }
    }
}

fn fetch_url<T: Displayer<U>, U: From<Text>>(
    url: &str,
    link: &ComponentLink<RequestLoader<T, U>>,
) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback =
        link.callback(|response: Response<U>| FetchMessage::Loaded(response.into_body()));

    FetchService::fetch(get_req, callback).unwrap()
}
```

The changes are evident from what we talked before, just generalizing stuff, we just need to move away the function that is used for `view`, indeed that is what we do next.

Let's create a displayer for the `markdown_visualizer`:

```rs
src/markdown_visualizer.rs
---
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use anyhow::Error;
use pulldown_cmark as pc;
use yew::html;
use yew::virtual_dom::VNode;
use yew::{web_sys, Html};

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

We simply moved the logic of the `view` function into the `displayer` and created the `BlogDisplayerComponent` type that is an specialization of the `RequestLoader` component over `BlogDisplayer`.

Finally, let's  update `src/lib.rs` with this component.

```rs
src/lib.rs
---
use markdown_visualizer::BlogDisplayerComponent;
use request_loader::RequestLoaderProps;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod markdown_visualizer;
mod request_loader;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<BlogDisplayerComponent>::new().mount_as_body_with_props(RequestLoaderProps {
        url: "/articles/test.md".to_string(),
    });
}
```

Let's compile it and test that everything is still working as usual

[state-5](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-5)

#### Adding list of article preview

Before getting to the component to create the preview list, we will add an end-point that returns all available articles.

So that it makes sense we add a new article in `static/articles/test2.md`:

```
static/articles/test2.md
---
# Quae nec quae

## Inhaesit gentis

Lorem markdownum illiusque numina monte fertur humi infelix: inmittor naribus
solvit. Angit ista agimus Lucifer, sive summae prima non quaeque. Urnam
tantaeque armataque detur despexitque aethere aequore pervia.

## Una venis creamur tactuque

Patetis pervigil membra ad vulnere **sed medio et** amantis illis Lernaeae
dominae praevisos. Interea regnum; simulat conplecti Aeacon futuro, et [uterum
consequar](http://formosiorreformet.net/minosmoriens.html) et mandat, conde sua
**circumfert** gelida. [Dum custos nimis](http://cetera.net/viderunt-suis.aspx)
profuit succidit te nube tura **vagitque**, lumina, in. Licet iubet heros sumpta
audete, secreta, mutavit perque noctis?

Caluere catenis membris praeter laedi. Sidera auras possit prodet longum longos
nomenque, tenebrosa quoque annua, [medio Mittor](http://reditusnisi.net/).

## Quid neque Nycteus

Movet ipse gemina moveant voluptas gemino sorores proicit inmutatque Phoebus
*fratrem pondere*. Superata Iuppiter; matrem potuissent sinit iacto aequor
iurgia reseminet. Regia veluti; mora visuraque languore longa hamatis refert
Narcissumque. Neque iugulum velocibus poscis potens, aut parte pectore vincere
fugam, esse. Quaerens Venus et illa fago hasta ex agmen foedera.

Amor cruentae [sacra saecula](http://www.gange-fatigatum.org/), non relinque
pinus, omnes rumorum: fixa notum ratis. Classis tactuque ex meus carinae legit,
[mihi](http://www.mavortis.org/) regnat fontem: quia exegi; onerosus numero,
amictus. Sermone est videt pronos vidit ab anxia mori conditus potentia.

## Impetus fulmina

Quod misit veluti meo, una est non conplexibus cantat neque. Iam fumant [cum
Mavortia](http://illam.com/) dolor, lyncum illa odiumque, in absentes tetigit,
aether.

    cpa.platformGrayscaleTransistor = secondaryOn;
    backsideProgrammingPaper = 89;
    var sink_scrolling = icq;
    https.diskUnicodeSql.pack(enterpriseTechnology);

Latus si dignus ligavit in aliter, et saepe. *Nam* celas refovet. Cum clausit
quod Messenia e quies meo iuga caelum auctor, esse duro intrata caterva namque.
Phaethontis erit hostis siqua, ut quam Polyxena dixisti inventum! Vires tu Lyaeo
dederat implevit serpentis vidit altis duos in quos mille verba, *exigua
aberant*.
```

We will use [Serde](https://serde.rs/) to serialize/deserialize the  object we will use to send and recieve from the server to encode the list of articles.

So, let's add it to our `Cargo.toml` in `[dependencies]` `serde = "^1.0"` and `serde_derive =  "^1.0"`(`serde` is the library and `serde_derive` has macros to really easily implement the serialize/deserialize methods for a struct).

Furthermore, we now will share the code for the struct that represents a list of articles between server and client, so that any update on the struct is automatically reflected in the client and server and we never desynchronize the API with the front-end. 

For sharing code between the two target(lib and bin) we need to have a crate-type of `lib` which then the compiler just do magic to compile to something usable in rust. So change the `crate-type` to `crate-type = ["cdylib", "lib"]`.(Remember you can read more [here](https://doc.rust-lang.org/reference/linkage.html))

`Cargo.toml` shall now look like this.

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

[dependencies.rocket_contrib]
version = "^0.4"
features = ["serve"]

[[bin]]
name = "server"

```

Let's create an struct to hold the struct for the article list that will be shared between server and client.

Let's create a new file `src/article_list.rs`

```rs
src/article_list.rs
---
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Articles {
    pub articles: Vec<String>,
}
```
Only interesting thing here is the `#[derive(Deserialize, Serialize, Debug)]`, the `Deserialize` and `Serialize` macros are part of the `serde_derive` lib that automatically implements methods to serialize and deserialize the struct, to many different formats(but we will use `Json` here because it is the easiest).

This struct would represent something like:

```json
{
  "articles": [string]
}
```

The `articles` field represent all the available articles. 

Now, we will add a constants file, to hold constants shared by server and client, it will be URIs

Add `src/constans.rs`:

```rs
src/constants.rs
---
pub const ARTICLE_LIST_URI: &str = "/article_list";
pub const ARTICLES_PATH: &str = "/articles";
pub const STATIC_URL: &str = "./static";
```

Now let's add an endpoint in the server to list the articles:

```rs
src/bin/server.rs
---
#![feature(proc_macro_hygiene, decl_macro)]
use blog::{article_list::Articles, constants};
use rocket::{get, routes};
use rocket_contrib::{json::Json, serve::StaticFiles};

use std::fs;

#[get("/article_list")]
fn list_articles() -> Json<Articles> {
    let articles_path = format!("{}{}", constants::STATIC_URL, constants::ARTICLES_PATH);
    let mut articles: Vec<_> = fs::read_dir(&articles_path)
        .unwrap_or_else(|_| {
            panic!(
                "Error ocurred while listing statics files in directory: {}",
                &articles_path
            )
        })
        .collect();

    articles.sort_by(|a, b| {
        a.as_ref()
            .unwrap()
            .file_name()
            .cmp(&b.as_ref().unwrap().file_name())
    });

    articles.reverse();

    let articles = articles
        .iter()
        .map(|res| res.as_ref().unwrap().file_name().into_string().unwrap())
        .collect();

    Json(Articles { articles })
}

fn main() {
    rocket::ignite()
        .mount("/", routes![list_articles])
        .mount("/", StaticFiles::from(constants::STATIC_URL))
        .launch();
}
```

So basically, this is:
1. Loading the contents of the article dir
1. Sorting the by file name, to keep some consistent ordering
1. Creating ` Json` with the struct we created before, which can be sent as a **JSON** (it's using the `Serialize`/`Deserialize` we added before in the back).

And we added the endpoint in the `main` function. Note that the name for the endpoint is given by the attribute `#[get("/article_list")]`. This attribute does what is expected, tells `Rocket` this function is a `GET` endpoint with `/article_list`.

And finally update `src/lib.rs` to make the `constant` and `article_list` modules public, so that the bin can use them.

```
src/lib.rs
---
use markdown_visualizer::BlogDisplayerComponent;
use request_loader::RequestLoaderProps;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod markdown_visualizer;
mod request_loader;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<BlogDisplayerComponent>::new().mount_as_body_with_props(RequestLoaderProps {
        url: "/articles/test.md".to_string(),
    });
}
```

let's compile and check that the endpoint is working correctly

Now let's add the list component

create the file `src/markdown_preview_list.rs`

Now compile and run.
```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build --release
cargo run
```

Now go to http://localhost:8000/article_list and see

```json
{"articles":["test2.md","test.md"]}
```

Printed in your browser.

[state-6](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-6)

#### Adding the article previewer

Well now, finally, let's add the `BlogPreviewListDisplayerComponent`, which componsed with `RequestLoader` will achieve a preview of the posts in the list.

Add `src/markdown_preview_list.rs`:

```
src/markdown_preview_list.rs
---
use crate::article_list::Articles;
use crate::markdown_visualizer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;

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
                                    </div>
                                }
                            })
                        }
                    }
                }
                _ => html! {<p>{"Error"}</p>},
            },
            None => html! {<p>{"Loading"}</p>},
        }
    }
}
```

If you look carefully, this is nothing special just re-using `BlogDisplayerComponent` with multiple urls. It gets the URLs from the `Articles` struct we defined before.

Now, let's use this component in `src/lib.rs` to use it as root, let's change change the mounted component

```
src/lib.rs
---
#![recursion_limit = "256"]

use markdown_preview_list::BlogPreviewListDisplayerComponent;
use request_loader::RequestLoaderProps;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod article_list;
pub mod constants;

mod markdown_preview_list;
mod markdown_visualizer;
mod request_loader;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<BlogPreviewListDisplayerComponent>::new().mount_as_body_with_props(RequestLoaderProps {
        url: constants::ARTICLE_LIST_URI.to_string(),
    });
}
```

Compile and run:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build --release
cargo run
```

Head to http://localhost:8000/ and you will see a list of preview of the articles.

[state-7](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-7)

