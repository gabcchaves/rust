use yew::prelude::*;

struct Container;
struct ComponentNoProps;

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

fn main() {
    yew::start_app::<Container>();
}
