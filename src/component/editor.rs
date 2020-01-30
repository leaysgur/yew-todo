use yew::prelude::*;

pub fn render_editor(
    value: &str,
    on_input_value: Callback<InputData>,
    on_click_add: Callback<ClickEvent>,
) -> Html {
    html! {
        <div>
            <input
            type="text"
            placeholder="Your todo..."
            value=&value
            oninput=on_input_value
            />
            <button
            type="button"
            onclick=on_click_add
            >{"add"}</button>
            </div>
    }
}
