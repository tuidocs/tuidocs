use std::collections::HashMap;

use tuidocs::PageManager;

struct TestPageManager {
    pages: HashMap<String, String>,
}

impl TestPageManager {
    pub fn new(pages: HashMap<String, String>) -> Self {
        Self { pages }
    }
}

impl PageManager for TestPageManager {
    fn get_page(&self, query: &str) -> Option<&String> {
        self.pages.get(query)
    }

    fn search(&self, query: &str) -> Vec<(String, u32)> {
        self.pages
            .iter()
            .map(|v| v.0.clone())
            .filter(|v| v.contains(query))
            .map(|v| (v, 0))
            .collect()
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
