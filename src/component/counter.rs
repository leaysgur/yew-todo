use yew::prelude::*;

pub fn render_counter(done_len: &usize, total_len: &usize) -> Html {
    html! {
        <p>{format!("{}/{} todo(s) are done!", done_len, total_len)}</p>
    }
}
