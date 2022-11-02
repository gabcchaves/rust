use yew::prelude::*;

/* Properties */
#[derive(PartialEq, Properties)]
struct Props {
    id: String,
    children: Children,
}

/* Components */
struct Container;
struct ComponentNoProps;
struct ComponentWithProps;

impl Component for Container {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <ComponentNoProps />
                <br/><br/>
                <ComponentWithProps id="1">
                    <p>{"This component has two properties."}</p>
                </ComponentWithProps>
            </>
        }
    }
}

impl Component for ComponentNoProps {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            {"This component has no properties."}
        }
    }
}

impl Component for ComponentWithProps {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id={ctx.props().id.clone()}>
                <strong>{format!("Component - {}", ctx.props().id.clone())}</strong>
                { ctx.props().children.clone() }
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Container>();
}
