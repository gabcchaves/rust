use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{"+1"}</button>
                <p>{ self.value }</p>
            </div>
        }
    }
}

struct App {
    section: u8,
}

enum MsgApp {
    Form,
}

impl Component for App {
    type Message = MsgApp;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            section: 0
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgApp::Form => {
                self.section = 1;
                true
            },
            MsgApp::Main => {
                self.section = 0;
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        match self.section {
            0 => html! {
                <>
                    <h1>{""}</h1>
                    <button onclick={link.callback(|_| MsgApp::Form)}>{"Form"}</button>
                </>
            },
            1 => html! {
                <>
                    <Form />
                </>
            },
        }
    }
}

struct Form {
    username: String,
}

impl Component for Form {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            username: String::new()
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <form>
                    <input type="text" placeholder="Username"/>
                    <input type="submit" value="Submit"/>
                </form>
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
