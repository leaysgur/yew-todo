use yew::{html, Component, ComponentLink, Html, ShouldRender};

pub struct Header {}

impl Component for Header {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Header {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <header>{"Yew TODO"}</header>
        }
    }
}
