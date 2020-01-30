use yew::prelude::*;

pub enum Msg {
    Update(String),
    Add,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[props(required)]
    pub on_add: Callback<String>,
}

pub struct Editor {
    link: ComponentLink<Self>,
    value: String,
    on_add: Callback<String>,
}
impl Component for Editor {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Editor {
            link,
            value: "".to_string(),
            on_add: props.on_add,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => self.value = val,
            Msg::Add => {
                self.on_add.emit(self.value.clone());
                self.value.clear();
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <input
                    type="text"
                    value={&self.value}
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value))
                />
                <button onclick=self.link.callback(|_| Msg::Add)>{"add"}</button>
            </div>
        }
    }
}
