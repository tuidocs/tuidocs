use std::collections::HashMap;

use tuidocs::PageManager;

struct TestPageManager {
    _pages: HashMap<String, String>,
}

impl TestPageManager {
    pub fn new(pages: HashMap<String, String>) -> Self {
        Self { _pages: pages }
    }
}

impl PageManager for TestPageManager {
    fn get_page(&self, _query: &str) -> Option<String> {
        todo!()
    }

    fn search(&self, _query: &str) -> Vec<(String, u32)> {
        todo!()
    }
}

fn main() {
    let page_manager = TestPageManager::new(
        vec![
            ("page1".to_string(), "this is page 1".to_string()),
            ("page2".to_string(), "this is page 2".to_string()),
        ]
        .into_iter()
        .collect(),
    );
    tuidocs::run(Box::new(page_manager));
}
