use crate::component::header::Header;

use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

pub enum Msg {
    Click,
}

pub struct App {
    clicked: bool,
    onclick: Callback<ClickEvent>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            clicked: false,
            onclick: link.callback(|_| Msg::Click),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click => {
                self.clicked = !self.clicked;
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.clicked {
            "Clicked ? true"
        } else {
            "Clicked ? false"
        };

        html! {
            <div>
                <Header />
                <button onclick=&self.onclick>{ button_text }</button>
            </div>
        }
    }
}
