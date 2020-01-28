use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, __: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>
                <h2>{"Yew TODO"}</h2>
            </header>
        }
    }
}
