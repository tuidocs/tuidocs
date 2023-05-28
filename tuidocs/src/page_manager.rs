pub trait PageManager {
    /// Get the page associated with the query.
    /// If the page does not exist, return `None`
    fn get_page(&self, query: &str) -> Option<String>;
    /// Search for pages.
    /// Return a list of pages with a line number to jump to
    fn search(&self, query: &str) -> Vec<(String, u32)>;
}