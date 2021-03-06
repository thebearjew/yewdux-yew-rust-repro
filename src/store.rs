use crate::modal::Modal;
use yewdux::prelude::{Changed, Reducer};

pub enum Action {
    ModalOpen,
    ModalClose,
    NoOp,
}

#[derive(Clone)]
pub struct Store {
    pub is_modal_open: bool,
}

impl Reducer for Store {
    type Action = Action;
    fn new() -> Self {
        Self {
            is_modal_open: false,
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::NoOp => false,
            Action::ModalOpen => {
                self.is_modal_open = true;
                true
            }
            Action::ModalClose => {
                self.is_modal_open = false;
                true
            }
        }
    }
}
