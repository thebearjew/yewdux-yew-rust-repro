mod modal;
mod store;
use crate::modal::Modal;
use yew::prelude::*;

struct App {}

impl Component for App {
    type Properties = ();
    type Message = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                {" hi" }
            </div>
        }
    }
}

pub fn app() -> Html {
    html! {
        <Modal>
            <div>{"Hi"}</div>
        </Modal>
    }
}

pub fn main() {
    yew::start_app::<App>();
}
