use super::searchresults::SearchResults;

pub struct SearchResponse {
    pub search_results: Vec<SearchResults>,
    pub total: i32,
    pub matches: i32,
}
