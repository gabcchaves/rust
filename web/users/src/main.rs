use yew::prelude::*;

enum Msg {
    Login,
    Register,
}


struct App {
    section: u8,
}

struct Header;
#[derive(PartialEq, Properties)]
struct HeaderProps {
    children: Children,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            section: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Login => {
                self.section = 1;
                true
            },
            Msg::Register => {
                self.section = 2;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <>
                <Header>
                    <button onclick={link.callback(|_| Msg::Login)}>{"Login"}</button>
                    <button onclick={link.callback(|_| Msg::Register)}>{"Register"}</button>
                </Header>
                if self.section == 1 {
                    <h1>{"Tela de LogIn"}</h1>
                } else if self.section == 2 {
                    <h1>{"Tela de Cadastro"}</h1>
                }
            </>
        }
    }
}

impl Component for Header {
    type Message = ();
    type Properties = HeaderProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header>
                { ctx.props().children.clone() }
            </header>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
