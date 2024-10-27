use std::str::FromStr;

use anyhow::Result;
use tui_scrollview::ScrollViewState;

use crate::wincast::{self, searchresponse::SearchResponse, searchresults::SearchResults};

#[derive(Debug, Default, PartialEq, Clone)]
pub enum Tab {
    #[default]
    Apps,
    Messages,
}

#[derive(Debug, Default, Clone)]
pub struct App<'a> {
    pub search_query: String,
    pub search_results: SearchResponse,
    pub selected_item: Option<&'a SearchResults>,
    pub selected_id: Option<usize>,
    pub scroll: ScrollViewState,
    pub active_tab: Tab,
    pub messages: Vec<String>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            search_query: String::new(),
            search_results: SearchResponse::new(),
            selected_item: None,
            selected_id: None,
            scroll: ScrollViewState::new(),
            active_tab: Tab::Apps,
            messages: Vec::new(),
        }
    }

    pub fn clone(&'a self) -> App<'a> {
        App {
            search_query: self.search_query.clone(),
            search_results: self.search_results.clone(),
            selected_item: self.selected_item.clone(),
            selected_id: self.selected_id.clone(),
            scroll: self.scroll.clone(),
            active_tab: self.active_tab.clone(),
            messages: self.messages.clone(),
        }
    }

    pub fn search(&mut self, query: String) -> Result<String> {
        self.search_results = wincast::search(&query[..])?;
        self.selected_item = None;
        self.selected_id = None;

        return Ok(query);
    }

    pub fn update_selection<'b>(&'a mut self, id: usize) {
        if id > 0 && id < self.search_results.search_results.len() {
            self.selected_id = Some(id);
            match self.search_results.search_results.get(id) {
                Some(item) => self.selected_item = Some(item),
                None => self.selected_item = None,
            }
        }
    }

    pub fn add_message(&mut self, message: &str) {
        self.messages.push(String::from_str(message).unwrap());
    }
}
