use crate::component::{Counter, Editor, Header, List};
use crate::state::State;
use yew::prelude::*;

pub enum Msg {
    Update(String),
    AddTodo,
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
        let state = State {
            todos: vec![],
            value: "".to_string(),
        };

        App { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => self.state.update(&val),
            Msg::AddTodo => self.state.add_todo(),
            Msg::ToggleTodo(idx) => self.state.toggle_todo(idx),
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <Header />
                {Editor(&self.state.value, self.link.callback(|e: InputData| Msg::Update(e.value)), self.link.callback(|_| Msg::AddTodo))}
                <hr />
                {List(&self.state.todos, |idx| self.link.callback(move |_| Msg::ToggleTodo(idx)))}
                <hr />
                {Counter(&self.state.done_len(), &self.state.total_len())}
            </div>
        }
    }
}
