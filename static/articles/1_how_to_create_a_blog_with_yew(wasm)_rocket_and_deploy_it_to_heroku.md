# How I built this blog using only Rust
## With Yew(WASM) and Rocket and deployed it to Heroku

As my first post in this blog I will go meta and write about how I built this blog, from start to finish. This will serve as some vague form of tutorial.

The "special" thing about this blog(if you may call it so) is that it is (almost!) written purely in [Rust](https://www.rust-lang.org/).

With that, I mean that both the front-end and back-end are written in this language. There are some glue code in JavaScript and HTML but it's negligible. 

So even if you have no interest in writting your own code for your blog, you might be interested in having a glimpse of how you too can be ridden of the JavaScript curse.


## Introduction

So first, credit where credit is due, I used [this post](https://www.steadylearner.com/blog/read/How-to-render-blog-posts-with-Rust-Yew-mounted-API) as a guiding light through building this website.

With that said, that post is a little outdated and uses web-std while here I will use web-sys. Also, this tutorial will encompass a greatest viewport from having nothing to getting a blog just like this working.

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

### Okay, but why?

Well, the answer comes down to "Rust is awesome". Yes, it's that simple
