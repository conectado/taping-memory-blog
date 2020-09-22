![Ferris](articles/img/cuddlyferris.svg#portrait)

# How I built this blog using only Rust (Part 1)
## With Yew(WASM) and Rocket and deployed it to Heroku

As my first post in this blog I will go meta and write about how I built this blog, from start to finish. This will serve as some vague form of tutorial.

The "special" thing about this blog(if you may call it so) is that it is (almost!) written purely in [Rust](https://www.rust-lang.org/).

With that, I mean that both the front-end and back-end are written in this language. There are some glue code in JavaScript and HTML but it's negligible. 

So even if you have no interest in writting your own code for your blog, you might be interested in having a glimpse of how you too can be ridden of the JavaScript curse.


## Introduction

So first, credit where credit is due, I used [this post](https://www.steadylearner.com/blog/read/How-to-render-blog-posts-with-Rust-Yew-mounted-API) as a guiding light through building this website.

With that said, that post is a little outdated and uses web-std while here I will use web-sys. Also, this tutorial will encompass a greatest viewport from having nothing to getting a blog just like this working.

### What we will be doing

Well, as the title said we will be writing a web blog (almost)purely in [Rust](https://www.rust-lang.org/). The web blog will fetch and read articles written in markdown and render them as you see them here.

The back-end will fetch the articles as part of the FileSystem(Not ideal, but it is easier than connecting to a DataBase that will have to do for now).

To build the front-end in this way we will use [WASM](https://webassembly.org/) which is simply an ISA that can be targetted by many compilers(such as rustc) and all modern browsers have a built-in VM to execute them. Anyways, we will talk more about the stack further down the road.

Finally we will be seeing how to deploy this application to [Heroku](https://heroku.com/) and later on how to integrate with [Github's Actions](https://github.com/features/actions).

### Why? No, seriously, Why?!

Well... that's a good question, specially when tools such as [jekyll](https://jekyllrb.com/) and [HUGO](https://gohugo.io/) exists.

And to answer that, while I could mention things that actually comes down to flexibility, I will point to some more fundamental advantages, but for that I'd rather talk about the stack I use before that.

### Stack

Well, the main technology behind this project is [Yew](https://yew.rs/). According to its webpage:

> Yew is a modern Rust framework for creating multi-threaded front-end web apps with WebAssembly.

>  * It features a component-based framework which makes it easy to create interactive UIs. Developers who have experience with frameworks like React and Elm should feel quite at home when using Yew.
>  * It achieves great performance by minimizing DOM API calls and by helping developers easily offload processing to the background using web workers.
>  * It supports JavaScript interoperability, allowing developers to leverage NPM packages and integrate with existing JavaScript applications.

Basically, it's an easy way to do front-end with Rust and it really is easy to get used to if you have used React before.

The framework is easy an intuitive, it has its pain points though, specially interacting with JavaScript

The back-end uses [Rocket](https://rocket.rs/) which is a very simple and powerful framework for web servers written in Rust. Which saddly, for a little more time will be running only in nightly, but that luckily will [change soon](https://github.com/SergioBenitez/Rocket/issues/19) and if we are lucky, we will start to see a lot more Rust in backend and I will be one happy person.

I have used a lot of other cool crates that really made everything easier. But I will talk about them later.

### Okay, but why?

Because every time I write JavaScript I cry myself to sleep.

No, but in all seriousness, the answer comes down to "Rust is awesome". Yes, it's that simple. Let me elaborate...

So I will not negate(yes! double negatives!) that my disgust for JS partly motivated my decision to use Yew to write this blog, I'd rather not rant against JavaScript(in this post) but better bask in Rust's glory.

So if you have dabbled into Rust for any amount of time you probably know about the guarantees, strong-typing, memory safety, thread safety, borrow checker, etc...

All these things, meaning having both static and dynamic guarantees make front-end more bearable. And Yew makes having all these benefits easy.

After using Yew to write this blog I can tell that it is really similar to React but with many more niceties, these being the guarantees we just talked about. 

With this I want to say, that sure, maybe using Jekyll would have been more inteligent(although I reject to touch Ruby even with a stick), but reading this article will be useful to get a feeling on how to write a front-end with Rust using Yew. Furthermore it's a good for getting some practice with Rust.

## Pre-requisities

### Knowledge

So, to follow this tutorial you will need some very basic HTML and JavaScript knowledge(Although you could copy paste the code, it's pretty boilerplatey).

I expect you to have read the [Rust Book](https://doc.rust-lang.org/stable/book/). Although the Rust used isn't that advanced I will not be explaining any concept explained in the book.

### Enviroment

You need to have [rustup](https://rustup.rs/) installed, I will walk through the rest of the dependencies.


## [Part 2](#articles/1_how_to_create_a_blog_with_yew(wasm)_rocket_and_deploy_it_to_heroku(part_2).md)
