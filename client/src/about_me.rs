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
            <div style="padding:1em;">
                <b>{"About Me"}</b>
                <p>{"I bang my head against the keyboard and sometimes a software happens"}</p>
                <p>{"I like that software better when it's written in Rust or C++"}</p>
                <p>{"In this blog I(will talk about anything that interest me, from system-programming to functional programming or anything else"}</p>
                <p>{"My pronouns are he/him and I'm from Argentina(thus the broken english)"}</p>
                <p>{"This blog is pretty much a WIP so you'll see it changing pretty often."}</p>
                <b>{"Contact"}</b>
                <p>{"Don't doubt to contact me for anything you want, I don't mind. Whether you want to talk to me about your project, you have a doubt or anything else I love talking to fellow programmers(as long as you're not rude)"}</p>
                <p>{"Use any of the following contacts, DMs to any of the platforms will be read(or mails)"}</p>
                <p>{"Email: taping-memory@protonmail.com"}</p>
                <p>{"Linkedin: https://www.linkedin.com/in/gabriel-alejandro-steinberg-40186a155"}</p>
                <p>{"Twitter: https://twitter.com/MemoryTaping"}</p>
            </div>
        }
    }
}
