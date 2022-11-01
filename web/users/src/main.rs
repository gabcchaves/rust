use yew::prelude::*;

enum Msg {
    Login,
    Register,
}

pub struct App;
pub struct ReusableHeader;
pub struct RegisterForm;
pub struct LoginForm;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container"}>
                <ReusableHeader />
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> Html {
        match msg {
            Msg::Login => {
                html! {
                    <ReusableHeader />
                    <Login />
                }
            }
        }
    }
}

impl Component for ReusableHeader {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onclick = ctx.link().batch_callback(|event: MouseEvent| Msg::Login);
        onclick.emit();
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
            </heder>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
