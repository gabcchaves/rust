use yew::prelude::*;

enum Msg {
    Login,
    Register,
}

pub struct App {
    section: i8, // 0 -> main; 1: login; 2: register
}
pub struct ReusableHeader;
pub struct RegisterForm;
pub struct LoginForm;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            section: 0
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container"}>
                <ReusableHeader />
                if self.section == 1 {
                    <Login />
                }
                // else if self.section = 2 {
                //     <Register />
                // }
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> u8 {
        match msg {
            Msg::Login => 1,
            Msg::Register => 2,
        }
    }
}

impl Component for ReusableHeader {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().callback(|_| Msg::Login);
        html! {
            <header class={"main-header"}> 
                <div class={"logo-area"}>
                    <h1>{"Compraqui"}</h1>
                </div>

                <div class={"search-area"}>
                    <input type={"text"} placeholder={"Pesquisar por produtos"}/>
                    <input type={"submit"} value={"Pesquisar"}/>
                </div>

                <div class={"log-options"}>
                    <button {onclick}>{"LogIn"}</button>
                </div>
            </header>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
