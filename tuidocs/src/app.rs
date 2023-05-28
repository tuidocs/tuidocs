use crate::{PageManager, error};

pub struct App {
    _page_manager: Box<dyn PageManager>,
}

impl App {
    pub fn new(page_manager: Box<dyn PageManager>) -> Self {
        Self {
            _page_manager: page_manager,
        }
    }

    pub fn run(self) -> Result<(), error::Error> {
        Ok(())
    }
}