# How I built this blog using only Rust (part 2)
## With Yew(WASM) and Rocket and deployed it to Heroku

Well, then, let's get started building a blog.

We will iteratively build the solution, little by little.

### Yew hello world
#### Preparing the environment

First, let's prepare our environment and get a `Hello world!` message in the browser using Yew.

Start by creating the project with `cargo`:

```bash
cargo init --lib blog
```

Now, edit `Cargo.toml` adding the following lines:

```toml
[lib]
crate-type = "cdylib"
```

This indicates that the lib will be compiled as a dynamic library that can link to other languages. Which makes sense, since we need interfacing with JavaScript.

To see more on why it's needed see [this wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/template-deep-dive/cargo-toml.html?highlight=cdylib#1--crate-type) and to see more on what each `crate-type` means see [this](https://doc.rust-lang.org/reference/linkage.html).

In `[dependencies]` add of `Cargo.toml`:

```toml
yew = "^0.17"
```

This is simply the [Yew](https://yew.rs/) library that we will be using to build the front-end.


Now `Cargo.toml` should be looking something like this:

```toml
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["<Your username>"]
edition = "2018"
   
[lib] 
crate-type = ["cdylib"]
 
[dependencies]
yew = "^0.17"
```

Now, we will install [wasm-pack](https://rustwasm.github.io), this will make the compilation and optimization of the package size extremely simple.

To install wasm-pack, let's use the [recommended method](https://rustwasm.github.io/wasm-pack/installer/):

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

And voilà, you have wasm-pack running.

Let's check that everything is running nicely, by building our "statics", although it won't do anything now.

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
```

This will output the built files into `./static/build` and the outputted JavaScript will be built to run in browser. For more options [see the wasm-pack docs](https://rustwasm.github.io/wasm-pack/book/commands/build.html).

#### Creating our first component: `HelloWorld`

Now we need to build a component that will show `Hello world!` in the browser.

For that we need a main function that can actually run in the browser and render components on the DOM.

So, let's do that by adding the following dependency:

```toml
wasm-bindgen = "^0.2"
```

`wasm-bindgen` is the lib that will do the magic binding between JavaScript and Rust.

In particular, it will let us create a main function such as that I talked about previously.

So, let's update `src/lib.rs` to look like the following so that we can finally show "Hello world!" in the browser:

```rs
src/lib.rs
---
use wasm_bindgen::prelude::*;
use yew::prelude::*;

struct HelloWorld;

impl Component for HelloWorld {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HelloWorld
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <p>{"Hello, world!"}</p>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<HelloWorld>::new().mount_to_body();
}
```

Okay, let's break this down in parts.

First we see the `wasm_bindgen(start)` attribute, this tells the function that it will be our stand-in main. To see the details of how this attribute really works [see this](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html).  

So, up until now I was talking about a component as what you would intuitively call one, a part of the DOM.

If you have used React before, you will know that there is a more precise meaning to this, so let's try to get a better grasp.

Components are the building blocks of Yew, they are structs that implements the trait `Component`, Yew then has the ability to render them as part of the DOM.

This trait defines how the internal state of the struct is managed throughout its lifetime and how it should be rendered. 

Also, it defines 2 types, `Properties`, which is the type it receives from its parent(similar to HTML attributes) and `Message` normally used for internal communication within a component. We will see more about these later.

For the methods in the trait I'll not enter in detail on how each of the trait's methods works, I will only give a high-level overview of those we use, for more info [see this](https://yew.rs/docs/en/concepts/components/)

* `create`: This represents when the component is first created, it receives the properties from its parent and is used as something similar to a constructor.
* `update`: When a callback occurs this method is called with the `Message` from the callback, more on this later.
* `change`: This is called when the parent change your properties(how rude 😟) with the new properties.
* `view`: This is the method that will be called to render the component in the DOM.

Note that `update` and `change` methods return a `ShouldRender` which is basically an alias for a `bool`, it tells **Yew** if it has to re-render the component due to the event that just occurred.

Another interesting thing to note is the `html` macro in the `view` method. This allows us to write an html-like syntax with the components, similar to [JSX](https://reactjs.org/docs/introducing-jsx.html), in this case we render `<p>{"Hello, world!"}</p>` were we are showing the string `Hello, world!` inside a `<p>` component that is equivalent to the normal HTML's `<p>` tag.
Also to use normal rust inside an `html` macro you need the `{` and `}`. To learn more about this macro refer [to the docs](https://yew.rs/docs/en/concepts/html/).

Putting all this together, this `HelloWorld` component simply render `<p>Hello, world!</p>` wherever it is put, without no properties and no changes throughout its lifetime.

Finally, we see the `run_app` function that creates an (App)[https://docs.rs/yew/0.17.3/yew/app/struct.App.html] which is a representation of a web-app, that is templetaized by our `HelloWorld` component, basically this `App` is a representation of the `HelloWorld` component, that as we see, after constructing(with `new`) we can call `mount_to_body` that renders the component directly in the body.

So, putting it all together, this renders `<p>Hello, world!</p>` in the browser.

Now let's ru-... Hold your horses, you thought you could do a `<p>Hello, world!</p>` in just 40 lines of code? Of course not.(I promise it pays off later)

We need to add the `index.html` to be served as default

Create the file `static/index.html` with this contents:

```html
static/index.html
---
<!DOCTYPE html>
<html lang="en">

  <head>
    <meta charset="utf-8">
    <meta http-equiv="X-UA-Compatible" content="IE=edge">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta name="description" content="blog">
    <title>blog</title>
    <script type="module">
      import init from "./build/wasm.js";
      init();
    </script>
  </head>
</html>

```
The only thing to notice here is the lines

```rs
import init from "./build/wasm.js";
init();
```

Which calls the function `init` that `wasm-pack` built for us(you can actually find the code in the `build/statics` directory, after building the code), this `init` takes charge of calling the function were we used the `wasm_bindgen(start)` attribute.

Now let's build the statics files.

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
```

Now `statics/index.html` should display `Hello, world!`, we only need to serve it now.

Now let's install `miniserve`, a server that just serves statics file in a given directory.

For that, we will need to use the nightly version of rust, since it doesn't work in stable(We need to use nightly for `rocket` later on anyways).

To change to nightly and install `miniserve`:

```bash
rustup override set nightly
cargo install miniserve --version 0.8.0
```

Now finally, let's get our hello world running.

```bash
miniserve ./static --index index.html -p 8888
```

Head over to http://localhost:8888/ and you should see our highly sought-after "Hello world"!

### Markdown visualizer component
#### Building a real component and rendering markdown

Now let's build our first real component, a markdown visualizer.

Add the following `dependency`:

```toml
pulldown_cmark = "^0.7"
```

`Cargo.toml` should look like this now

```toml
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["<Your username>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
yew = "^0.17"
wasm-bindgen = "^0.2"
pulldown-cmark = "^0.7"
```

[pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) is a markdown parser that comes with an HTML renderer.

Let's use it to build our `markdown_visualizer` component. 

```rs
src/markdown_visualizer.rs
---
use pulldown_cmark as pc;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::web_sys;

pub struct MarkdownVisualizer;

impl Component for MarkdownVisualizer {
    type Message = ();
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        MarkdownVisualizer
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            {view_markdown("# This is bold!")}
        }
    }
}

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
```

Here, we create a component `MarkdownVisualizer`, the only interesting method is the `view` which renders `# This is bold!` by calling the function `view_markdown`.

Yew has 2 options to interface with Web API, `web_sys` and `stdweb`, we will use `web_sys` since it has official support from the Rust/WASM WG(for more info [see this](https://yew.rs/docs/en/getting-started/choose-web-library/).

`web_sys` expose most of the browser's API, thus letting us create elements directly in the DOM, in fact `create_markdown_container` does just this, leveraging `web_sys` it creates `div` of class **markdown-body** in which we then set the HTML contents.

We need to do it like this because if we simply pass the `String` representing the `HTML` rendering of our markdown to the `html` macro, it just renders the string explicitly, the browser never parses the string.

If we use div's `set_inner_html` the browser interprets it as HTML, parses it and renders it.

`view_markdown` does the following in this order.

1. Creates a `pulldown_cmark` parser for the passed value.
1. Create `html_output` as a buffer for the parser
1. parses `value` and convert it to `html` by using `push_html`, loads the result into the buffer `html_output`
1. Creates a div to hold the results by calling `create_markdown_container`
1. Sets the HTML contents of the div to `html_output`
1. Creates Node reference to the `div`
1. Returns a virtual reference to the node(what Yew can render)

With this this component should render our markdown.

A little note before continuing, see how easily we included a Rust library(`pulldown_cmark`) in the front-end, think how powerful it's writing an application/back-end lib and being able to "freely" use it in the front-end.

Finally, let's get rid of the `HelloWorld` component and mount this component to body instead.

```rs
src/lib.rs
---
use markdown_visualizer::MarkdownVisualizer;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod markdown_visualizer;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<MarkdownVisualizer>::new().mount_to_body();
}
```

Let's build & serve this. Great, by now you should see the very simple markdown in the browser!.

### Adding the requestloader component
#### Using the fetch service

Now, we want to serve an article instead of just hardcoding the string(although for now we will hardcode the URL)

We will deal with Http request and responses so let's add a module to deal with that. To the dependencies in `Cargo.toml` add `http = "^0.2"`.
Also, let's add `Anyhow` to handle errors `anyhow = "^1.0"`

```
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["conectado <gabrielalejandro7@gmail.com>"]
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
```

now create a new file `src/request_loader.rs`

```
src/request_loader.rs
---
use yew::prelude::*;
use yew::services::{fetch::FetchTask, FetchService};
use yew::virtual_dom::VNode;
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader {
    props: RequestLoaderProps,
    fetch_task: FetchTask,
    display_value: Option<Result<String, Error>>,
    link: ComponentLink<Self>,
}

pub trait Displayer {
    fn display(text: &Option<String>) -> VNode;
}

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps {
    pub url: String,
}

pub enum FetchMessage {
    Loaded(Result<String, Error>),
}

impl Component for RequestLoader {
    type Properties = RequestLoaderProps;
    type Message = FetchMessage;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let _fetch_task = fetch_article_list(&props.url, &link);
        RequestLoader {
            props,
            fetch_task: _fetch_task,
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
            self.fetch_task = fetch_article_list(&self.props.url, &self.link);
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            {
                match &self.display_value {
                    Some(response) => match response {
                        Ok(value) => view_markdown(value),
                        Error => html!{{"Error!"}},
                    },
                    None => html!{{"Loading..."}}
                }
            }
        }
    }
}

fn fetch_article_list(url: &str, link: &ComponentLink<RequestLoader>) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback = link.callback(|response: Response<Result<String, Error>>| {
        FetchMessage::Loaded(response.into_body())
    });

    FetchService::fetch(get_req, callback).unwrap()
}
```

Let's update `src/markdown_visualizer.rs` to discard the component we won't be using

 ```
src/markdown_visualizer.rs
---
use pulldown_cmark as pc;
use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew::web_sys;

fn create_markdown_container() -> web_sys::Element {
    let window = web_sys::window().expect("Can't find window");
    let document = window.document().expect("Can't find document");
    let div = document.create_element("div").expect("Couldn't create div");
    div.set_class_name("markdown-body");
    div
}

pub fn view_markdown(value: &str) -> Html {
    let parser = pc::Parser::new(value);
    let mut html_output = String::new();
    pc::html::push_html(&mut html_output, parser);

    let div = create_markdown_container();

    div.set_inner_html(&html_output);

    let node = web_sys::Node::from(div);
    VNode::VRef(node)
}
```

Let's add a test article. I used [Lorem Markdownum](https://jaspervdj.be/lorem-markdownum/) to generate the random markdown.

Create a file in `static/articles/test.md` with the following contents

```
static/articles/test.md
---
# Cecropis ego illae venientia tamen

## Ducunt surgentibus saxo colorem annis

Lorem markdownum Caenis Polydoreo rapta, res unum Abas specie rubor. Stetit non.
Interea omnibus: loci petam una vimine exsultat fert lassata Parcarum tamen.
Sanguine sibi iam, urnaque superi *non* pererrat vultus notasti. Humo
*Melanchaetes gravidis illius* consiliique: sidera in succensa creber?

Mihi et orba terras est! Nec si palmas arentia priorum capillis etiam, cera
notam. Inquit secreta [distulit pectora](http://dixeratsed.com/iove-fixis.php).
Esse nec, de Titania? Quod amplexa caligine arceat corpusque memoranda certe
aquarum.

## Ceciderunt isse suaque tuisque mittitur

Quo hoc movit annis; abibas at Achillis rexque, huic, ames. Quid vestigia
exhausta lente fatetur **lentos** pectore unus.

Opus vinaque gerens. Natorum genetrici credit nefas et regnat Mycenida pericula
actis signaque *hinc simulacraque* excipit sed. Rota depositoque thalamos meque
Mycenae iusto nescio **maestaeque**, adspexit deprendere ne
[vita](http://enim.com/modo.html), quod silvasque dimovit aspiciunt ab inquit.
Inpia rettulit nisi, **buxo** quod, nunc *deo* movitque quem! Cum male gremio
sicco, cancer, sum ubi *omnes*, patrem bracchia.

## Enim heres stantis fratrem et sine praecordia

Fata maius ostendit hoc medicas potest femina, adempta maternae et quoque mater
nymphe caput. Non sanguine vires, dea non constitit dixere videri reluxit,
ineunt paene tigris admonita *Appenninigenae*.

> Fecit et **alba**, relabi nunc ecce facies alto murmur a te. En falsa pedes
> mater genetricis angues deficis moras erat: natae aether succincta opem, fatum
> ipsa cantare votorum. Mora nec nec divitior Cnosia etiam insidias toto contra
> posito nascuntur [mentis](http://www.meum.org/) nuntia et quemque, avertit.
> Limbo pars mihi patrios et blandita lentae cito esset caelestia bipennis
> suntque moriens primusque cumba.

## Poplite sparsit

Calamis matre Andraemone mora collo oracula certamine iugales damna deusque.
Orbis notam mea, radice ambierantque secundo Semina honore mox possemque. Estote
**ire** aequoris, si regia litem praeceps ursae. Aves vimque spe aetate haec
idem et cibus vestrumque vulnus aestus brevi, bracchia **Peleus** melioris,
Indis.

- Diros modestos
- Parte Liriope
- Dracones a

Cum urbem; ait incenduntque tumens. In dares tanti toto Coronida scopuloque
meaque illas aliquos moram; caede aeno divite auctor. Heu fecit quoque *regno
dea* Iove cum moriens maternos evellere magno; soli dotaliaque. Sine *nobis*,
est dum laeva et torreri, virtute et edidit tuens quaterque, **moenibus**,
admonita.
```

And now let's add the `RequestLoader` as the root component

```rs
src/lib.rs
---
use request_loader::{RequestLoader, RequestLoaderProps};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod markdown_visualizer;
mod request_loader;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<RequestLoader>::new().mount_as_body_with_props(RequestLoaderProps {
        url: "/articles/test.md".to_string(),
    });
}
```

Now let's create a preview list of articles

For that we will need a proper back-end, so let's add [Rocket](https://rocket.rs/)

First let add to the `Cargo.toml` a new `bin` as a target for compilation.

```toml
[[bin]]
name = "server"
```

then let's add `rocket = "^0.4"` to the dependencies.

and rocket_contrib to serve statics easily

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
authors = ["conectado <gabrielalejandro7@gmail.com>"]
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

### Generalizing the RequestLoader component

Now, let's generalize the `RequestLoader` component


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
    phantom: std::marker::PhantomData<T>,
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
        let fetch_task = fetch_article_list(&props.url, &link);
        RequestLoader {
            props,
            phantom: std::marker::PhantomData,
            fetch_task,
            display_value: None,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let FetchMessage::Loaded(text) = msg;
        self.display_value = Some(text);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.display_value = None;
            self.props = props;
            self.fetch_task = fetch_article_list(&self.props.url, &self.link);
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

fn fetch_article_list<T: Displayer<U>, U: From<Text>>(
    url: &str,
    link: &ComponentLink<RequestLoader<T, U>>,
) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback =
        link.callback(|response: Response<U>| FetchMessage::Loaded(response.into_body()));

    FetchService::fetch(get_req, callback).unwrap()
}
```

Let's update the markdown_visualizer to work with this

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

Let' update `src/lib.rs`

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

### Adding blog preview list

Now let's add a new article in `static/articles/test2.md`

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

Let's add the crates for these next section `serde = "^1.0"`, `serde_derive =  "^1.0"` add a name to the lib and add the `lib` type

```
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["conectado <gabrielalejandro7@gmail.com>"]
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

Let's create an struct to hold the article list, let's create a new file `src/article_list.rs`

```rs
src/article_list.rs
---
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Articles {
    pub articles: Vec<String>,
}
```

let's add a file to share constants `src/constants.rs`

```rs
src/constants.rs
---
pub const ARTICLE_LIST_URI: &str = "/article_list";
pub const ARTICLES_PATH: &str = "/articles";
pub const STATIC_URL: &str = "./static";
```

Now let's add an endpoint in the server to list the articles

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

And finally update `src/lib.rs`

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

```
src/markdown_preview_list.rs
---
use yew::virtual_dom::VNode;

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
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> VNode {
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

To make our life easier let's create a root component in `src/root.rs`

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

Let's us this component in `src/lib.rs`

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

And for the new styles to work add bootstrap and fontawesome

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

Now let's add routing for this page

First, add `yew-router="^0.14"` to `Cargo.toml`

```
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["conectado <gabrielalejandro7@gmail.com>"]
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

[[bin]]
name = "server"
```
Now, let's update the `root` component

```
src/routes.rs
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
                            {"Taping Memory"}
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

And let's add the routes in `src/routes.rs`


```
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

And finally add the module to `src/lib.rs`

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
mod routes;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Root>::new().mount_as_body();
}
```

Finally, let's also update the preview

```rs
src/markdown_preview_list.rs
---
use yew::virtual_dom::VNode;

use crate::article_list::Articles;
use crate::blog_displayer::BlogDisplayerComponent;
use crate::request_loader::Displayer;
use crate::request_loader::RequestLoader;
use crate::routes::AppRoute;
use crate::spinner::spinner;
use anyhow::Error;
use yew::format::Json;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub type BlogPreviewListDisplayerComponent =
    RequestLoader<BlogPreviewListDisplayer, Json<Result<Articles, Error>>>;

pub struct BlogPreviewListDisplayer;

impl Displayer<Json<Result<Articles, Error>>> for BlogPreviewListDisplayer {
    fn display(text: &Option<Json<Result<Articles, Error>>>) -> VNode {
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
            None => spinner(),
        }
    }
}
```

Now let's add syntax highlight in `src/markdown_visualizer.rs`

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

let's add the query selector feature to `Cargo.toml`

```rs
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["conectado <gabrielalejandro7@gmail.com>"]
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

and let's change `static/index.html` to servee hljs

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

And now let's change the first article for somthing with a little rust code

```rs
static/articles/test.md
---

\`\`\`rs
fn main() {
  println!("hello world!");
}
\`\`\`
```

And tada! we are done, next blogspot we will see how to deploy this

