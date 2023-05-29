use std::collections::HashMap;

use faker_rand::en_us::names::FullName;

use tuidocs::PageManager;

struct ExamplePageManager {
    pages: HashMap<String, String>,
}

impl ExamplePageManager {
    pub fn new(pages: HashMap<String, String>) -> Self {
        Self { pages }
    }
}

impl PageManager for ExamplePageManager {
    fn get_page(&self, query: &str) -> Option<&String> {
        self.pages.get(query)
    }

    fn search(&self, query: &str) -> Vec<(String, u16)> {
        self.pages
            .iter()
            .map(|v| v.0.clone())
            .filter(|v| {
                v.to_lowercase()
                    .replace(" ", "")
                    .contains(&query.to_lowercase().replace(" ", ""))
            })
            .map(|v| (v, 10))
            .collect()
    }
}

fn main() {
    let page_manager = ExamplePageManager::new(
        {
            let mut out: HashMap<String, String> = HashMap::new();
            for _ in 0..20 {
                out.insert(
                    rand::random::<FullName>().to_string(),
                    lipsum::lipsum(500).replace(". ", ".\n"),
                );
            }
            out
        }
        .into_iter()
        .collect(),
    );
    tuidocs::run(Box::new(page_manager));
}
