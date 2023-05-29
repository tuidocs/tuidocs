use std::{collections::HashMap, fmt::format};

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

    fn search(&self, query: &str) -> Vec<(String, u16)> {
        self.pages
            .iter()
            .map(|v| v.0.clone())
            .filter(|v| v.contains(query))
            .map(|v| (v, 10))
            .collect()
    }
}

fn main() {
    let page_manager = TestPageManager::new(
        {
            let mut out: HashMap<String, String> = HashMap::new();
            for i in 0..20 {
                out.insert(format!("Page {}", i + 1), lipsum::lipsum(200));
            }
            out.insert("test page".to_string(), {
                let mut out_ = String::new();
                for j in 0..50 {
                    out_.push_str(&format!("Line {}\n", j + 1));
                }
                out_
            });
            out
        }
        .into_iter()
        .collect(),
    );
    tuidocs::run(Box::new(page_manager));
}
