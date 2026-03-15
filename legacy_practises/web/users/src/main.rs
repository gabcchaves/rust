use yew::prelude::*;

enum MsgSection {
    Main,
    Login,
    Register,
}

struct App {
    section: u8,
}

struct Header;
struct Main;
struct Login;
struct Register;

#[derive(PartialEq, Properties)]
struct HeaderProps {
    children: Children,
}
#[derive(PartialEq, Properties)]
struct MainProps {
    children: Children,
}
#[derive(PartialEq, Properties)]
struct LoginProps {
    children: Children,
}
#[derive(PartialEq, Properties)]
struct RegisterProps {
    children: Children,
}

impl Component for App {
    type Message = MsgSection;
    type Properties = ();
    
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            section: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgSection::Main => {
                self.section = 0;
                true
            },
            MsgSection::Login => {
                self.section = 1;
                true
            },
            MsgSection::Register => {
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
                    <div class="login-area">
                        <h1 onclick={link.callback(|_| MsgSection::Main)}>{"Compraqui"}</h1>
                    </div>
                    <div class="log-options">
                        <button onclick={link.callback(|_| MsgSection::Login)}>{"Login"}</button>
                        <button onclick={link.callback(|_| MsgSection::Register)}>{"Register"}</button>
                    </div>
                </Header>
                if self.section == 0 {
                    <Main>
                        <h2>{"Página principal"}</h2>
                    </Main>
                } else if self.section == 1 {
                    <Login>
                        <h2>{"Página de login"}</h2>
                        <form method="POST">
                            <input type="email" placeholder="E-mail"/>
                            <input type="password" placeholder="Senha"/>
                            <input type="submit" value="Entrar"/>
                        </form>
                    </Login>
                } else if self.section == 2 {
                    <Register>
                        <h2>{"Página de cadastro"}</h2>
                        <form method="POST">
                            <input type="text" placeholder="Nome completo"/>
                            <input type="email" placeholder="Endereço de e-mail"/>
                            <input type="password" placeholder="Senha"/>
                            <input type="password" placeholder="Confirmar senha"/>
                            <input type="submit" value="Cadastrar"/>
                        </form>
                    </Register>
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
                <>
                    { ctx.props().children.clone() }
                </>
            </header>
        }
    }
}

impl Component for Main {
    type Message = ();
    type Properties = MainProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { ctx.props().children.clone() }
            </>
        }
    }
}

impl Component for Login {
    type Message = ();
    type Properties = LoginProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { ctx.props().children.clone() }
            </>
        }
    }
}

impl Component for Register {
    type Message = ();
    type Properties = RegisterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                { ctx.props().children.clone() }
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
