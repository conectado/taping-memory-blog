# How I built this blog using only Rust (part 2)
## Preparing the environment and Visualizing markdown in the browser

Well, then, let's get started building a blog.

We will iteratively build the solution.

### Yew hello world
#### Preparing the environment

First, let's prepare our environment and get a `Hello world!` message in the browser using Yew.

Start by creating the project with `cargo`:

```bash
cargo init --lib blog
```

Edit `Cargo.toml` adding the following lines:

```toml
[lib]
crate-type = "cdylib"
```

This indicates that the lib will be compiled as a dynamic library that can link to other languages. Which makes sense, since we need interfacing with JavaScript.

To see more on why it's needed see [this wasm-pack documentation](https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/template-deep-dive/cargo-toml.html?highlight=cdylib#1--crate-type) and to see more on what each `crate-type` means see [this](https://doc.rust-lang.org/reference/linkage.html).

Next add this dependency:

```toml
yew = "^0.17"
```

This is simply the [Yew](https://yew.rs/) library that we will be using to build the front-end.

Now `Cargo.toml` should look something like this:

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

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-0)

Now, we will install [wasm-pack](https://rustwasm.github.io), this will make the compilation and optimization of the package size extremely simple.

To install wasm-pack, let's use the [recommended method](https://rustwasm.github.io/wasm-pack/installer/):

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

And voilà, you have wasm-pack running.

Let's check that everything is running nicely, by building our "statics", although it won't do anything for now.

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
```

This will output the built files into `./static/build`. For more options [see the wasm-pack docs](https://rustwasm.github.io/wasm-pack/book/commands/build.html).

#### Creating our first component: `HelloWorld`

Now we need to build a component that will show `Hello world!` in the browser.

For that we need a main function that can actually run in the browser and render components to the DOM.

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

Okay, let's break this down into parts.

First we see the `wasm_bindgen(start)` attribute above `run_app`, this tells the associated function that it will be our stand-in main. To see the details of how this attribute really works [see this](https://rustwasm.github.io/wasm-bindgen/reference/attributes/on-rust-exports/start.html).  

So, up until now I was talking about a component as what you would intuitively call one, a part of the DOM.

If you have used React before, you will know that there is a more precise meaning to this, so let's try to get a better grasp.

Components are the building blocks of Yew, they are structs that implements the trait `Component`, Yew then has the ability to render them as part of the DOM.

This trait defines how the internal state of the struct is managed throughout its lifetime and how it should be rendered. 

Also, it defines 2 types, `Properties`, which is the type it receives from its parent(similar to HTML attributes) and `Message` normally used for internal communication within a component. We will see more about these later.

For the methods in the trait I'll not enter in detail on how each of the trait's methods works, I will only give a high-level overview of those we use, for more info [see this](https://yew.rs/docs/en/concepts/components/)

* `create`: This represents when the component is first created, it receives the properties from its parent and is used as something similar to a constructor.
* `update`: When a callback occurs this method is called with the `Message` from the callback, more on this later.
* `change`: This is called when the parent change your properties(how rude 😟) with new ones.
* `view`: This is the method that will be called to render the component in the DOM.

Notice that `update` and `change` methods return a `ShouldRender` which is basically an alias for a `bool`, it tells **Yew** if it has to re-render the component due to the event that just occurred.

Another interesting thing to note is the `html` macro in the `view` method. This allows us to write an html-like syntax with the components, similar to [JSX](https://reactjs.org/docs/introducing-jsx.html), in this case we render `<p>{"Hello, world!"}</p>` were we are showing the string `Hello, world!` inside a `<p>` component that is equivalent to the normal HTML's `<p>` tag.
Also to use normal rust inside an `html` macro you need the `{` and `}`. To learn more about this macro refer [to the docs](https://yew.rs/docs/en/concepts/html/).

Putting all this together, this `HelloWorld` component simply render `<p>Hello, world!</p>` wherever it is put, without properties and changes throughout its lifetime.

Finally, we see the `run_app` function that creates an (App)[https://docs.rs/yew/0.17.3/yew/app/struct.App.html] which is a representation of a web-app, that is templatized with our `HelloWorld` component, basically this `App` is a representation of the `HelloWorld` component, that as we see further on in the statement, after constructing(with `new`) we can call the method `mount_to_body`, this renders the component directly into the body.

So, putting it all together, this renders `<p>Hello, world!</p>` in the `body`.

Now let's run-... Hold your horses, you thought you could do a `<p>Hello, world!</p>` in just 40 lines of code? Of course not.(I promise it pays off later)

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

```html
import init from "./build/wasm.js";
init();
```

Which calls the function `init` that `wasm-pack` built for us(you can actually find the code in the `build/statics` directory, after building the code), this `init` takes charge of calling the function were we used the `wasm_bindgen(start)` attribute.

Now let's build the statics files:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
```

`statics/index.html` should display `Hello, world!`, we only need to serve it now.

We will install `miniserve`, a server that just serves statics file in a given directory.

For that, we need to use the nightly version of rust since it doesn't work in stable(We need to use nightly for `rocket` later on anyways).

To change to nightly and install `miniserve`:

```bash
rustup override set nightly
cargo install miniserve --version 0.8.0
```

Finally, let's get our hello world running.

```bash
miniserve ./static --index index.html -p 8888
```

Head over to http://localhost:8888/ and you should see our highly sought-after "Hello world"!

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-1)

### Markdown visualizer component
#### Building a real component and rendering markdown

Now let's build our first real component, a markdown visualizer.

Add the following `dependency`:

```toml
pulldown_cmark = "^0.7"
```

`Cargo.toml` should look like this:

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

Let's use it to build our `markdown_visualizer` component, create the file `src/markdown_visualizer` with the following contents:

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

We need to create a div with `inner_html` set to the **HTML** we just rendered, if we were to simply output something like `html!{{html_output}}` the browser would simply render the `html_output` explicitly, meaning `<p>something</p>` would look like **<p>something</p>** instead of a paragraph with **something**.

`view_markdown` does the following:

1. Creates a Parser for the passed value.
1. Creates `html_output` as a buffer for the parser
1. parses `value` and convert it to `html` by using `push_html`, loads the result into the buffer `html_output`
1. Creates a div to hold the results by calling `create_markdown_container`
1. Sets the HTML contents of the div to `html_output`
1. Creates Node reference to the `div`
1. Returns a virtual reference to the node(what Yew can render)

With this, this component should render "# This is bold!" as the html translation of this markdown.

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

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-2)

### Adding the `RequestLoader` component
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

The reason for he generic name will be made clear later. `RequestLoader` for a given URL, will send a request to that URL and when the response arrives, it will display `view_markdown` applied to the contents of the body of the response.

Next you can see the contents of the `request_loader`:

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

It has `props` which we will look into later but it receives them from its parent.

`fetch_task` is a handle to the request we are expecting a response from or has already loaded.

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

And then we have this enum:

```rs
pub enum FetchMessage {
    Loaded(Result<String, Error>),
}
```

Which will hold the result of the request.

In the implementation for the `Component` trait we see how the `FetchMessage` and `RequestLoaderProps` are assigned to the type `Message` and `Props` for the trait, respectively.

In the `create` method, we initialize the request using `fetch_url` and create the struct. 

The `change` method as we talked before is called when the parent change the property of its child, in this case if the parent decides "Well, actually I want to request another URL" we empty the holder of the request response and restart the fetch process, but only if the URL is actually different, otherwise we don't do anything. Of course if the URL changes, we need to re-render the component so we return `true` otherwise it doesn't need to re-render and for performance reasons we return `false`.

Before seeing the `update` and `display` method, let's explore the `fetch_url` function. This uses the Yew's service `FetchService`, which is basically a way to easily call into the browser's [fetch API](https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API/Using_Fetch). A quick look at this function will tell you that it creates a **GET** request using the `http` crate's  `Request`. Then it creates a callback using the `ComponentLink` which puts the result of the request in the `FetchMessage` enum when it is called, the `fetch_url` function finally calls the `FetchService` and returns the task of the on-going fetch call. Which we just need to hold so that we don't drop the reference, but any information out of this fetch we get through the callback.

Let's take one more look into the creation of the callback for the fetch:

```rs
let callback = link.callback(|response: Response<Result<String, Error>>| {
    FetchMessage::Loaded(response.into_body())
});
```

This `link` variable is a `ComponentLink<RequestLoader>`, as we said before, `ComponentLink` is a way to send messages to an instance of the component. In this case, the `callback` method let us create a callback, that when called will in turn call the `update` function of the component with the return of the callback.

Now, we can finally analyze the `update` method, this is precisely what is called when the response arrives(because the callback we created with the `link` is called), a quick look at it tells you that when it's called it unpacks the result into `display_value` and asks for a re-render of the component.

The `view` method isn't that much different from the `markdown_visualizer` just more handlers for the different stages of the request.

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

Create a file in `static/articles/test.md` with the following contents:

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

And now let's add the `RequestLoader` as the root component:

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

Here, just note that we are using `mount_as_body_with_props` which allows us to pass props to the component we are mounting as the body.

Compile and run and enjoy your rendered article:

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
miniserve ./static --index index.html -p 8888
```

[Here you can see how the code should be looking now](https://github.com/conectado/yew-tutorial-web-blog-states/tree/state-3)

## [Part 3](#articles/3_how_to_create_a_blog_with_yew(wasm)_rocket_and_deploy_it_to_heroku(part_3).md)

