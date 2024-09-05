use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{app::App, utils::greet};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(9),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let title =
        Paragraph::new(Text::styled(greet(), Style::default().fg(Color::Red))).block(title_block);

    frame.render_widget(title, chunks[0]);

    let search_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let search = Paragraph::new(Text::styled(
        format!("Search: {}", &app.search_query.clone()),
        Style::default(),
    ))
    .block(search_block);

    frame.render_widget(search, chunks[1]);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let mut list_items = Vec::<ListItem>::new();

    for (id, search_result) in app.search_results.search_results.iter().enumerate() {
        let default_list_item = ListItem::new(Text::styled(
            format!("{}", search_result.name),
            Style::default().fg(Color::default()),
        ));
        if let Some(selected_id) = &app.selected_id {
            if id == *selected_id {
                list_items.push(ListItem::new(Text::styled(
                    format!("{}", search_result.name),
                    Style::default().fg(Color::Black).bg(Color::LightGreen),
                )))
            } else {
                list_items.push(default_list_item)
            }
        } else {
            list_items.push(default_list_item);
        }
    }

    let list = List::new(list_items).block(list_block);

    frame.render_widget(list, chunks[2]);
}
