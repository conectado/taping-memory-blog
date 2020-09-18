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
[state-0](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-0)

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

[state-1](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-1)

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
    Html::VRef(node)
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

[state-2](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-2)

### Adding the requestloader component
#### Using the fetch service

Now, we want to serve an article instead of just hard-coding the string(although for now we will hard-code the URL)

We will deal with HTTP requests and responses, so let's add a module to deal with that. To the dependencies in `Cargo.toml` add `http = "^0.2"`.
Also, let's add `Anyhow` to handle errors `anyhow = "^1.0"`.

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
crate-type = ["cdylib"]

[dependencies]
yew = "^0.17"
wasm-bindgen = "^0.2"
pulldown-cmark = "^0.7"
http = "^0.2"
anyhow = "^1.0"
```

Now create a new file `src/request_loader.rs` to hold our component `RequestLoader`, this component will replace our `MarkdownVisualizer`.

The reason for the generic name will be made clear later. `RequestLoader` for a given URL, will send a request to that URL and when the response arrives, it will display with `markdown_visualizer` the contents of the body of the response.

```
src/request_loader.rs
---
use crate::markdown_visualizer::view_markdown;
use anyhow::Error;
use http::{Request, Response};
use yew::prelude::*;
use yew::services::{fetch::FetchTask, FetchService};
use yew::{format::Nothing, html, Component, ComponentLink, Html, ShouldRender};

pub struct RequestLoader {
    props: RequestLoaderProps,
    fetch_task: FetchTask,
    display_value: Option<Result<String, Error>>,
    link: ComponentLink<Self>,
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

fn fetch_url(url: &str, link: &ComponentLink<RequestLoader>) -> FetchTask {
    let get_req = Request::get(url).body(Nothing).unwrap();
    let callback = link.callback(|response: Response<Result<String, Error>>| {
        FetchMessage::Loaded(response.into_body())
    });

    FetchService::fetch(get_req, callback).unwrap()
}
```

Now, our component has a state given by the fields of its struct

```rs
pub struct RequestLoader {
    props: RequestLoaderProps,
    fetch_task: FetchTask,
    display_value: Option<Result<String, Error>>,
    link: ComponentLink<Self>,
}
```

It has `props` which we will look into later but it recieves them from its parent.

`fetch_task` is a handle to the `fetch_task` related to the request that it is either trying to or has loaded.

`display_value` will hold the response or lack thereof.

`link` is a handle to the `ComponentLink` which is a way to send messages to the Component. In this case we will use it to tell the component when the request finished.

The props for this component is:

```rs
#[derive(Properties, Debug, Clone, PartialEq)]
pub struct RequestLoaderProps {
    pub url: String,
}
```

Which, is basically the URL for the request.

And then we have this enum

```rs
pub enum FetchMessage {
    Loaded(Result<String, Error>),
}
```

Which will hold the result of loading the request.

In the implementation for the component type we see how the `FetchMessage` and `RequestLoaderProps` are assigned to the type `Message` and `Props` for the trait, respectively.

In the `create` method, we initialize the request using `fetch_url` and create the struct. The change method as we talked before is called when the parent change the property of its child, in this case if the parent decides "Well, actually I want to request another URL".

In that case we empty the holder of the request response and re_start the fetch process, but only if the URL is actually different, otherwise we don't do anything. Of course if the URL changes, we need to re-render the component so we return `true` otherwise it doesn't need to re-render and for performance reasons we return `false`.

Before seeing the `update` and `display` method, let's explore the `fetch_url` function. This uses the `yew` service `FetchService`, which is basically a way to easily call into the browser's [fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch). A quick look at this function, will tell you that it creates a **GET** request using the `http`'s library `Request`. Then it creates a callback using the `ComponentLink` which simply puts the result  in the `FetchMessage` enum when it is called, and finally it calls the `FetchService` and returns the task of the on-going fetch call. Which we just need to hold so that we don't drop the reference, but any information out of this fetch we get through the callback.

Let's take one more look into the creation of the callback for the fetch:

```rs
let callback = link.callback(|response: Response<Result<String, Error>>| {
    FetchMessage::Loaded(response.into_body())
});
```
This link is a `ComponentLink<RequestLoader>`, as we said before, `ComponentLink` is a way to send messages to an instance of the component. In this case, the `callback` method let us create a callback, that when called, it will then call the `update` function of the component with the return of the callback.

Now we can finally see the update function, that's precisely called when the response arrives(because the callback we created with the link is called), a quick look at it tells you that when it's called it de-structure the result into `display_value` and asks for a re-render of the component.

Then the `view` method isn't that much different from the `markdown_visualizer` just more handlers for the different stages of the request.

Let's update `src/markdown_visualizer.rs` to discard the component we won't be using, since we are using the `request_loader` we only keep the `view_markdown` function:

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

Let's add a test article to render with the `request_loader`. 

I used [Lorem Markdownum](https://jaspervdj.be/lorem-markdownum/) to generate the random markdown.

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

Here, just note that we are using `mount_as_body_with_props` which allows us to pass props to the component we are passing.

Compile and run and bask in the glory of your rendered markdown:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
miniserve ./static --index index.html -p 8888
```

[state-3](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-3)

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

Let's use this component in `src/lib.rs`

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

The root component use some styles so let's add them.

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

[state-8](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-8)

#### Adding routing

Lastly, we will add routing for the page, since this is a SPA we will need routing.

Routing, simply is a way to indicate the APP using the URL which component should be shown, making it work smoothly with the backward and forward buttons of the browser and making it easy redirecting to other components in the webpage. Luckily, there is already a library for Yew that does this for us.

First, add `yew-router="^0.14"` to `Cargo.toml`

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

Let's add a `src/routes.rs` file that will hold an enum with all the available routes in the application.

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

It is quite similar(we could say equivalent) to how a router in the back-end functions, just it's routing the front-end.

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
Well, a slight improvement on styles were added, but more importantly we added 2 things: a `RouterAnchor`, this is just like an `a` element which instead of `href` it takes `route` which can use a Routing enum as long as its element has a `to` attribute. So as the `AppRoute::List` is the home(related to the `/` URL) the element:

```rs
<RouterAnchor<AppRoute> route={AppRoute::List}>
  <i class="fas fa-home" style="font-size: 2em; color: white;"></i>
</RouterAnchor<AppRoute>>
```

Is just like a link to the homepage(if we ever change the URL in the enum the change would be reflected here). [More on the RouterAnchor](https://docs.rs/yew-router/0.14.0/yew_router/components/struct.RouterAnchor.html).

And then the `Router` element, to understand more about how it works see [the documentation](https://docs.rs/yew-router/0.14.0/yew_router/router/struct.Router.html) and this [guide](https://yew.rs/docs/en/concepts/router/), but basically is an element with a property `render` which takes a closure(wraped by a `Router::render`) that takes an `AppRoute` that simply represent the current state of the `AppRoute` based on the `URL` and it should return the component you want to render based on this. 

For the `AppRoute::List` it simply displays the preview list, and for anything matching `AppRoute::ViewPost(article)`(meaning an URL pattern akin to `/#articles/{article}` ) it will display the blog component with the URL corresponding to that article.


Finally, let's also update the preview list to have links to the articles:

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

And with this you could test your now navigable blog.

[state-9](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-9)

#### Adding syntax highlighting

As the last thing let's add syntax highlighting for the markdown visualizer

For highlighting I just found a JS library called `hljs`. So we have to deal wiith the boundary between JS and Rust.

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

`extern "C"` means, this is an external function, you will not find the definition here. Then the `#[wasm_bindgen]` attribute tells wasm_bindgen to do its magic and create the bindings with JS.

Then, we notice:

```rs
let code_blocks = div.query_selector_all("pre code").unwrap();
for i in 0..code_blocks.length() {
    hljs::highlightBlock(JsValue::from(code_blocks.get(i).unwrap()));
}
```

In the `view_markdown` code. Well, this is part of the Browser API, which we have access to thanks to `web_sys`. `query_selector_all("pre code")` give us all the child nodes of the div and allows us to highlight that node passing it to `hljs`. More on [this API](https://developer.mozilla.org/en-US/docs/Web/API/Document/querySelectorAll).

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
Finally, we need to add the js lib to the HTML:

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

\`\`\`rs
fn main() {
  println!("hello world!");
}
\`\`\`
```

Notice that you need to erase the backslashes.

And tada! we are done, next blogspot we will see how to deploy this blogspot to heroku.

[state-10](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-10)
