use anyhow::Result;

use crate::wincast::{self, searchresponse::SearchResponse, searchresults::SearchResults};

pub enum CurrentScreen {
    Home,
    Search,
    Exit,
}

impl CurrentScreen {
    pub fn clone(&self) -> CurrentScreen {
        match self {
            CurrentScreen::Home => CurrentScreen::Home,
            CurrentScreen::Search => CurrentScreen::Search,
            CurrentScreen::Exit => CurrentScreen::Exit,
        }
    }
}

pub struct App<'a> {
    pub current_screen: CurrentScreen,
    pub search_query: String,
    pub search_results: SearchResponse,
    pub selected_item: Option<&'a SearchResults>,
    pub selected_id: Option<usize>,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            current_screen: CurrentScreen::Search,
            search_query: String::new(),
            search_results: SearchResponse::new(),
            selected_item: None,
            selected_id: None,
        }
    }

    pub fn clone(&'a self) -> App<'a> {
        App {
            current_screen: self.current_screen.clone(),
            search_query: self.search_query.clone(),
            search_results: self.search_results.clone(),
            selected_item: self.selected_item.clone(),
            selected_id: self.selected_id.clone(),
        }
    }

    pub fn toggle_screen(&mut self) {
        match self.current_screen {
            CurrentScreen::Home => self.current_screen = CurrentScreen::Search,
            CurrentScreen::Search => self.current_screen = CurrentScreen::Home,
            _ => {}
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
}
