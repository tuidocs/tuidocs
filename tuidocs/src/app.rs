use crate::{PageManager, State};

pub struct App {
    pub page_manager: Box<dyn PageManager>,
    pub state: State,
}

impl App {
    pub fn new(page_manager: Box<dyn PageManager>) -> Self {
        Self {
            page_manager,
            state: State::Reading,
        }
    }
}
