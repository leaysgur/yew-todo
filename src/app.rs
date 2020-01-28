use crate::component::header::Header;
use crate::state::{State, Todo};
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
                <div>
                    <input
                        type="text"
                        placeholder="Your todo..."
                        value=&self.state.value
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                    />
                    <button
                        type="button"
                        onclick=self.link.callback(|_| Msg::AddTodo)
                    >{"add"}</button>
                </div>
                <hr />
                {self.render_lists(&self.state.todos)}
                <hr />
                <p>{format!("{}/{} todo(s) are done!", self.state.done_len(), self.state.total_len())}</p>
            </div>
        }
    }
}

impl App {
    fn render_lists(&self, todos: &Vec<Todo>) -> Html {
        html! {
            <ul>
            { for todos.iter().enumerate().map(|(idx, todo)| {
                html! {
                    <li>
                        <span>{idx + 1}</span>
                        <input
                            type="checkbox"
                            checked=todo.done
                            onclick=self.link.callback(move |_| Msg::ToggleTodo(idx))
                        />
                        {todo.title.clone()}
                    </li>
                }
            }) }
            </ul>
        }
    }
}
