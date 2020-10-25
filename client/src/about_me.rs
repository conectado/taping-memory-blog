use yew::prelude::*;

pub struct AboutMe;

impl Component for AboutMe {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AboutMe
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div style="padding:1em; text-align: center; font-size: 1.1em;">
                <b style="font-weight: 1000;">{"About Me"}</b>
                <div>
                    <img src="./imgs/kitten-white.png#profile" />
                </div>
                <p>{"Name: Gabriel Steinberg"}</p>
                <p>{"Location: Argentina, CABA"}</p>
                <p>{"I bang my head against the keyboard and sometimes a software happens 🌟"}</p>
                <p>{"I like that software better when it's written in Rust or C++ 🦀"}</p>
                <p>{"In this blog I will talk about anything that interest me, from system-programming to functional programming or anything else"}</p>
                <p>{"My pronouns are he/him and I'm from Argentina(thus, the broken english)"}</p>
                <p>{"This blog is pretty much a WIP so you'll see it changing pretty often."}</p>
                <b style="font-weight: 1000;">{"Contact"}</b>
                <p>{"Don't doubt to contact me for anything you want, I don't mind. Whether you want to talk to me about your project, you have a doubt or anything else I love talking to fellow programmers(as long as you're not rude)"}</p>
                <p>{"Also you can contact me for any possible project! (paid or unpaid)"}</p>

                <p>{"Use any of the following contacts, DMs to any of the platforms will be read(or mails)"}</p>
                <a href="mailto:taping-memory@protonmail.com" style="display: block;">
                    <i class="icon-mail-squared" style="display: inline;"/>
                    <p style="display: inline;">
                        {"Regarding this blog"}
                    </p>
                </a>
                <a href="mailto:gabrielalejandro7@gmail.com" style="display: block;">
                    <i class="icon-mail-squared" style="display: inline;" />
                    <p style="display: inline;">
                        {"Regarding work/projects or personal"}
                    </p>
                </a>
                <p style="display: block;">
                    <a href="https://twitter.com/MemoryTaping" style="display: inline;">
                        <i class="icon-twitter" />
                    </a>
                    <a href="https://www.linkedin.com/in/gabriel-alejandro-steinberg-40186a155" style="display: inline;">
                        <i class="icon-linkedin"/>
                    </a>
                    <a href="https://github.com/conectado/" style="display: inline;">
                        <i class="icon-github"/>
                    </a>
                </p>
                <p/>
                <a href="https://www.behance.net/akooros">
                    {"Profile art by akooros"}
                </a>
            </div>
        }
    }
}
