use yew::prelude::*;

pub struct App {
    link: ComponentLink<Self>,
    message: String,
}

pub enum Msg {
    UpdateMessage(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App {
            link,
            message: "Hello, world!".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateMessage(new_message) => {
                self.message = new_message;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <p>{ &self.message }</p>
                <button onclick=self.link.callback(|_| Msg::UpdateMessage("Hello, Yew!".to_string()))>{"Change Message"}</button>
            </div>
        }
    }
}
