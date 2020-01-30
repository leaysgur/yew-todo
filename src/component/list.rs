use crate::state::Todo;
use yew::prelude::*;

pub fn render_list<F>(todos: &Vec<Todo>, on_click_done: F) -> Html
where
    F: Fn(usize) -> Callback<ClickEvent>,
{
    html! {
        <ul>
        { for todos.iter().enumerate().map(|(idx, todo)| {
            html! {
                <li>
                    <span>{idx + 1}</span>
                    <input
                        type="checkbox"
                        checked=todo.done
                        onclick=on_click_done(idx)
                    />
                    {todo.title.clone()}
                </li>
            }
        }) }
        </ul>
    }
}
