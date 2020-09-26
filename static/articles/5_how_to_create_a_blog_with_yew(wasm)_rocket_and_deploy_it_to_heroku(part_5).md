# How I built this blog using only Rust (part 5)
## Highlighting the code

### Deploying to Heroku and conclusions

Here I'll very quickly and briefly tell you how I finally deployed this blog to heroku.

First create an account in [Heroku](https://id.heroku.com/login), you don't need a credit card or anything like that a free account will do.

Next, select **Create App**:

![Create App](articles/img/heroku_create_app.png#square)

Select a name, I used "taping-memory-test-blog" use whichever region.


Now, go to the **settings** tab:

![Settings](articles/img/heroku_setting_tab.png#article)

And select **Add buildpack**:

![Add buildpack](articles/img/heroku_add_buildpack.png#article)

Add these buildpacks in this order(You can re-order them later):

```
https://github.com/conectado/heroku-buildpack-rust-wasm-pack.git
https://github.com/heroku/heroku-buildpack-nodejs#v170
```

![Buildpacks](articles/img/heroku_buildpacks.png#article)


Next install heroku CLI: https://devcenter.heroku.com/articles/heroku-cli

Login to heroku CLI:

```bash
heroku login
```

If the project wasn't already a git project:

```bash
git init
```

And then:

```bash
heroku git:remote -a <your-app-name>
```

Add a file `package.json`:

```json
package.json
---
{
  "name": "blog",
  "version": "1.0.0",
  "main": "index.js",
  "license": "MIT",
  "scripts": {
    "build": "wasm-pack build --target web --out-name wasm --out-dir ./static/build && cargo build --release"
  }
}
```

And also a file `Procfile`:

```
Procfile
---
web: ./target/release/server
```

Now do this:

```
git add .
git commit -m "First heroku deploy"
git push heroku master
```

And voila~ no you can visit your very own published blog.

### Conclusions

Okay, so this are my thoughts now.

First, Yew states in its repo that it's not production ready, so this is just a glimpse of what we will be able to do with it. However, I think there's some really important take-away.

In the last 10 years there has been a popularization of front-end languages(meaning JavaScript) carried to the back-end, the excuse for brining such an awful thing to the back-end  has always been "you only need to learn one language".(I might be a little hyperbolic here)

However, WASM brings us a new possibility, carry the back-end languages to the front-end, you have only one language for the whole stack(plus everything that is nice with Rust). I believe this is really cool and open a whole new world o possibilities, meaning that Rust can really be a full-stack language, Front-end, Back-end, System programming, Embedded programming... It's not there yet but it clearly has the potential it just need more time for the libraries to mature.


Now, this blog has changed a lot since the version I showed you here, I changed the styles, Rocket for Actix(due to performance) and a couple of refactors and I plan to continue improving it. At the moment, the articles are in the same repo as the blog which is not convenient, and having the same crate for both the server and client is not ideal so this will be changing in the near-time, also the optimizations such as minification, Webpack, etc... are still pending.

Furthermore, my plans are having the repo for this blog as an easy to fork project. So that anyone can create a blog easily out of this one.

If you want to see the latest version of this blog you should [look here](github.com/conectado/taping-memory-blog) and of course PRs and issues are more than welcome!

If you want to send any feedback you can do it anywhere I posted this or send it to taping-memory@protonmail.com

I hope this was useful to you. See you next blog post.
