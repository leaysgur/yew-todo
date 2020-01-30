use crate::component::{Counter, Editor, Header, List};
use crate::state::State;
use yew::prelude::*;

pub enum Msg {
    AddTodo(String),
    ToggleTodo(usize),
}

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State { todos: vec![] };

        App { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddTodo(val) => self.state.add_todo(val),
            Msg::ToggleTodo(idx) => self.state.toggle_todo(idx),
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                {Header()}
                <Editor on_add=self.link.callback(|v: String| Msg::AddTodo(v)) />
                <hr />
                {List(&self.state.todos, |idx| self.link.callback(move |_| Msg::ToggleTodo(idx)))}
                <hr />
                {Counter(&self.state.done_len(), &self.state.total_len())}
                <hr />
            </div>
        }
    }
}
