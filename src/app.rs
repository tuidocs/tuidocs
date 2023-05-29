use crate::{PageManager, State};

pub struct App {
    pub page_manager: Box<dyn PageManager>,
    pub state: State,
    pub input: String,
    pub last_input: String,
    pub selected_entry: usize,
    pub scroll: u16,
}

impl App {
    pub fn new(page_manager: Box<dyn PageManager>) -> Self {
        Self {
            page_manager,
            state: State::Reading,
            input: String::new(),
            last_input: String::new(),
            selected_entry: 0,
            scroll: 0,
        }
    }
}
