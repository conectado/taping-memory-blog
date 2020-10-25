![Ferris](articles/img/cuddlyferris.svg#portrait)

# How I built this blog using only Rust (Part 1)
## With Yew(WASM) and Rocket and deployed it to Heroku

So, yeah, welcome to my blog, hi!

As my first post in this blog I will go meta and write about how I built this blog, from start to finish. This will serve as some sort of tutorial.

The "special" thing about this blog(if you may call it so) is that it is (almost!) written purely in [Rust](https://www.rust-lang.org/).

With that I mean that both the front-end and back-end are written in this language. There is some CSS and some glue HTML but it's negligible. 

So even if you have no interest in writing your own code for your blog, you might be interested in having a glimpse of how you too can get rid of the JavaScript curse.


## Introduction

So first, credit where credit is due, I used [this post](https://www.steadylearner.com/blog/read/How-to-render-blog-posts-with-Rust-Yew-mounted-API) as a guiding light through building this website.

With that said, that post is a little outdated and uses web-std while here I will use web-sys. Also I use a native library to go from markdown to HTML. However, those blog posts are still relevant and are worth looking at and helped me a lot getting started.

### What we will be doing

Well, as the title said we will be writing a web blog (almost) purely in [Rust](https://www.rust-lang.org/). The web blog will fetch and read articles written in markdown and render them as you see them here.

The back-end will fetch the articles as part of the FileSystem(Not ideal, but it is easier than connecting to a DataBase that will have to do for now), we won't be doing anything fancy since the focus is in the front-end.

To build the front-end in Rust we will use [WASM](https://webassembly.org/) which is simply an ISA that can be targeted by many compilers(such as [rustc](https://doc.rust-lang.org/rustc/index.html)) and most modern browsers have a built-in VM to execute it. Anyways, we will talk more about the stack further down the road.

Finally we will be seeing how to deploy this application to [Heroku](https://heroku.com/).

### Why? No, seriously, Why?!

Well... That's a good question, specially when tools such as [jekyll](https://jekyllrb.com/) and [HUGO](https://gohugo.io/) exists.

And to answer that, while I could mention flexibility, that doesn't precisely justify using Rust in the front-end but having my own back-end instead of a purely static web-site. However there are certain advantages for using Rust but I would rather talk about the stack we will be using before that.

### Stack

Well, the main technology behind this project is [Yew](https://yew.rs/). According to its webpage:

> Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.

>  * It features a component-based framework which makes it easy to create interactive UIs. Developers who have experience with frameworks like React and Elm should feel quite at home when using Yew.
>  * It achieves great performance by minimizing DOM API calls and by helping developers easily offload processing to the background using web workers.
>  * It supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.

Basically, it's an easy way to do front-end with Rust and it really is easy to get used to if you have used React before.

For the back-end we will use [Rocket](https://rocket.rs/) which is a very simple and powerful framework for web servers written in Rust. Which saddly, for a little more time will be running only in nightly, but that luckily will [change soon](https://github.com/SergioBenitez/Rocket/issues/19).

I have used a lot of other cool crates that really made everything easier. But I will talk about them later.

### Okay, but why?

Because every time I write JavaScript I cry myself to sleep.

No, but in all seriousness, the answer comes down to "Rust is awesome". Yes, it's that simple. Let me elaborate...

I will not negate that my disgust for JS partly motivated my decision to use Yew to write this blog, but rather than ranting against JavaScript I'd rather bask in Rust's glory.(Also, Rust is not the only alternative to JS)

So if you have dabbled into Rust for any amount of time you probably know about the guarantees, strong-typing, memory safety, thread safety, borrow checker, etc...

All these things, meaning having both static and dynamic guarantees make front-end more bearable. And Yew makes having all these benefits easy.

After using Yew to write this blog I can tell that it is really similar to React but with all the niceties we just talked about. 

With this I want to say, that sure, maybe using Jekyll would have been more intelligent(although I refuse to touch Ruby even with a stick), but reading this article will be useful to get a feeling on how to write a front-end with Rust using Yew. Furthermore it's a good for getting some practice with Rust.

## Pre-requisites

### Knowledge

So, to follow this tutorial you will need some very basic HTML and JavaScript knowledge(Although you could copy paste the code, it's pretty boilerplatey).

I expect you to have read the [Rust Book](https://doc.rust-lang.org/stable/book/). Although the Rust used isn't that advanced I will not be explaining any concept explained in the book.

Also this tutorial is mostly oriented for people who are used to back-end development, so most of the back-end concepts won't be explained but I will try to explain most of the front-end concepts used.

### Enviroment

You need to have [rustup](https://rustup.rs/) installed, I will walk through the rest of the dependencies.

### Preview

[Here](https://taping-memory-test-blog.herokuapp.com/) you can see how the blog will look like at the end of this tutorial.

## [Part 2](#articles/2_how_to_create_a_blog_with_yew(wasm)_rocket_and_deploy_it_to_heroku(part_2).md)
