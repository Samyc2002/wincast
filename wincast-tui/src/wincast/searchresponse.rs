use super::searchresults::SearchResults;

#[derive(Debug, Default, Clone)]
pub struct SearchResponse {
    pub search_results: Vec<SearchResults>,
    pub total: i32,
    pub matches: i32,
}

impl SearchResponse {
    pub fn new() -> SearchResponse {
        SearchResponse {
            search_results: Vec::new(),
            total: 0,
            matches: 0,
        }
    }

    pub fn clone(&self) -> SearchResponse {
        let mut search_results = Vec::new();
        for search_result in &self.search_results {
            search_results.push(search_result.clone());
        }

        SearchResponse {
            search_results,
            total: self.total,
            matches: self.matches,
        }
    }
}
