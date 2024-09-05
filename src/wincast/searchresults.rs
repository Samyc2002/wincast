pub struct SearchResults {
    pub name: String,
    pub path: String,
    pub icon: String,
    pub search_type: String,
}

impl SearchResults {
    pub fn clone(&self) -> SearchResults {
        SearchResults {
            name: self.name.clone(),
            path: self.path.clone(),
            icon: self.icon.clone(),
            search_type: self.search_type.clone(),
        }
    }
}
