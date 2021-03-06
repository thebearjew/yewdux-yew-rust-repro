use gloo::console::*;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlDivElement, SvgElement};
use yew::events::MouseEvent;
use yew::prelude::*;
use yewdux::prelude::*;

use super::icon::IconXCircle;
use crate::store::{Action, Store};

#[derive(Properties, PartialEq, Clone, Default)]
pub struct ModalProps {
    // NOTE: It is super important that we prefix this prop with prop_or_default macro, this allows
    // us to have dispatch injected by yewdux rather than passing it in manually
    #[prop_or_default]
    pub dispatch: DispatchProps<ReducerStore<Store>>,
    #[prop_or_default]
    pub children: Children,
}

impl WithDispatchProps for ModalProps {
    type Store = ReducerStore<Store>;

    fn dispatch(&self) -> &DispatchProps<Self::Store> {
        &self.dispatch
    }
}

pub type Modal = WithDispatch<ModalBase>;
pub struct ModalBase;
impl Component for ModalBase {
    type Message = ();
    type Properties = ModalProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let modal_host = document()
            .get_element_by_id("portal-target")
            .expect("Missing element with ID #portal-target");

        let is_modal_open = ctx.props().dispatch.state().is_modal_open;
        let on_close_modal = ctx.props().dispatch.callback(|e: MouseEvent| {
            let t1: Option<EventTarget> = e.target();
            let t2 = t1.clone();
            let div = t1.and_then(|t| t.dyn_into::<HtmlDivElement>().ok());
            let svg = t2.and_then(|t| t.dyn_into::<SvgElement>().ok());
            let div_id = div.map(|d| d.id()).unwrap_or_else(|| "".to_string());
            let svg_id = svg.map(|s| s.id()).unwrap_or_else(|| "".to_string());
            if div_id == "modal-background" || div_id == "close-icon" || svg_id == "close-icon" {
                Action::ModalClose
            } else {
                Action::NoOp
            }
        });

        let maybe_modal_children = if is_modal_open {
            html! {
                <>
                    <div
                        onclick={&on_close_modal}
                        // id={id_modal_background}
                        // class={classes_background}
                    >
                    </div>

                    // Modal
                    <div id={id_modal_window} >
                        // Content
                        <div id={id_modal_content} class={classes_relative_container}>
                            { ctx.props().children.clone()}
                        </div>
                    </div>
                </>
            }
        } else {
            html! {}
        };
        create_portal(maybe_modal_children, modal_host)
    }
}
