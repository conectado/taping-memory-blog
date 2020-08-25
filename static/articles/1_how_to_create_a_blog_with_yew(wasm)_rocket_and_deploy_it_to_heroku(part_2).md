# How I built this blog using only Rust (part 2)
## With Yew(WASM) and Rocket and deployed it to Heroku

```bash
cargo init --lib blog
```

Edit `Cargo.toml`

Add

```toml
[lib]
crate-type = "cdylib"
```

and in `[dependencies]` add

```
yew = "^0.17"
```

Now it should look something like this

```toml
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["conectado <gabrielalejandro7@gmail.com>"]
edition = "2018"
   
[lib] 
crate-type = ["cdylib"]
 
[dependencies]
yew = "^0.17"
```

Now, let's install wasm-pack, let's use the [recommended method](https://rustwasm.github.io/wasm-pack/installer/)

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Now let's build the statics

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
```

If it all work it fine let's add something to the body of the app.

Add `wasm-bindgen` to the depenendencies

```toml
wasm-bindgen = "^0.2"
```

Now, update `src/lib.rs` like this so that we can have our *hello world* done. (We will go through the code in a moment)

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

Now let's add the `index.html` to be served as default

Create the file `static/index.html` with the contents

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

Let's build the changes

```bash
wasm-pack build --target web --out-name wasm --out-dir ./static/build
```

Now let's install `miniserve` to quickly serve the static files for now.

For that, we will need to use the nightly version of rust, since it doesn't work in stable(We need that for `rocket` later on anyways)

```bash
rustup override set nightly
cargo install miniserve --version 0.8.0
```

Now let's serve finally, the hello world

```bash
miniserve ./static --index index.html -p 8888
```

Now let's build our first real component, a markdown visualizer.

Now let's add `pulldown-cmark = "^0.7"` to our dependencies to parse and render markdown.

It should look like this now

```toml
Cargo.toml
---
[package]
name = "blog"
version = "0.1.0"
authors = ["conectado <gabrielalejandro7@gmail.com>"]
edition = "2018"

[lib]
crate-type = ["cdylib"]

[dependencies]
yew = "^0.17"
wasm-bindgen = "^0.2"
pulldown-cmark = "^0.7"
```

This is the component

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

and let's update `src/lib.rs` to get rid of the `HelloWorld` component


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
