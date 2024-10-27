use anyhow::Result;
use app::Search;
use std::error::Error;
use utils::index;

pub mod app;
pub mod utils;
pub mod wincast;

fn main() -> Result<(), Box<dyn Error>> {
    index();
    iced::run("Search Bar", Search::update, Search::view)?;
    return Ok(());
}
