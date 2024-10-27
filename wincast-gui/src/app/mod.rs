use iced::widget::{Button, Column, Text, TextInput};

use crate::{
    utils::launch,
    wincast::{search, searchresults},
};

#[derive(Default)]
pub struct Search {
    query: String,
    results: Vec<searchresults::SearchResults>,
}

#[derive(Debug, Clone)]
pub enum Message {
    ContentChanged(String),
    AppLaunch(String),
}

impl Search {
    pub fn view(&self) -> Column<Message> {
        let mut col = Column::new();
        col = col.push(
            TextInput::new("Search Query here...", &self.query).on_input(Message::ContentChanged),
        );

        for result in self.results.clone() {
            let button =
                Button::new(Text::new(result.name)).on_press(Message::AppLaunch(result.path));

            col = col.push(button);
        }
        col.into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.query = content.clone();
                let apps = search(&content.clone());
                match apps {
                    Ok(apps) => self.results = apps.search_results.clone(),
                    Err(_) => self.results = Vec::new(),
                }
            }
            Message::AppLaunch(app_path) => {
                launch(app_path);
                std::process::exit(0);
            }
        }
    }
}
