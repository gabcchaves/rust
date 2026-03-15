use yew::prelude::*;

pub struct App;
pub struct Header;
pub struct Form;
pub struct Footer;

impl Component for App {
    type Message = ();
    type Properties = ();
    
    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class={"container"}>
                <Header />
                <Form />
                <Footer />
            </div>
        }
    }
}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <header class={"main-header"}>
                <div class={"logo-area"}>
                    <h1>{"Compraqui"}</h1>
                </div>
                <div class={"search-area"}>
                    <input type={"text"} placeholder={"Pesquisar por produtos"}/>
                    <input type={"submit"} value={"Pesquisar"}/>
                </div>
            </header>
        }
    }
}

impl Component for Form {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <form method={"POST"}>
                <input type={"email"} placeholder={"E-mail"}/>
                <input type={"password"} placeholder={"Senha"}/>
                <input type={"submit"} value={"Entrar"}/>
            </form>
        }
    }
}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <footer class={"main-footer"}>
                <strong>{"Experimento da estrutura Yew"}</strong>
            </footer>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
