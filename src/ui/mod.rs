use ratatui::{
    layout::{Constraint, Direction, Flex, Layout, Rect, Size},
    style::{Color, Style, Stylize},
    symbols,
    text::Text,
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, StatefulWidget, Tabs},
    Frame,
};
use tui_scrollview::ScrollView;

use crate::app::{App, Tab};

pub struct FArea {
    height: u16,
    width: u16,
}

pub fn get_frame_area() -> Rect {
    return Rect::new(0, 0, 130, 100);
}

pub fn get_search_area() -> FArea {
    let area = FArea {
        height: 40,
        width: 130,
    };
    return area;
}

pub fn get_tab_index(tab: Tab) -> usize {
    if tab == Tab::Apps {
        return 0;
    } else if tab == Tab::Messages {
        return 1;
    } else {
        return 0;
    }
}

pub fn center(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}

pub fn render_popup(frame: &mut Frame) {
    let area = center(
        frame.area(),
        Constraint::Percentage(20),
        Constraint::Length(3),
    );
    let popup = Paragraph::new("Popup content").block(Block::bordered().title("Popup"));
    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

pub fn ui(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(get_search_area().height),
            Constraint::Length(0),
        ])
        .split(center(
            frame.area(),
            Constraint::Percentage(80),
            Constraint::Percentage(80),
        ));

    let search_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let search = Paragraph::new(Text::styled(
        format!("Search: {}", &app.search_query.clone()),
        Style::default(),
    ))
    .block(search_block);

    frame.render_widget(search, chunks[0]);

    let tab_data = vec!["Apps", "Messages"];

    let tab_block = Block::default()
        .borders(Borders::NONE)
        .style(Style::default());

    let tabs = Tabs::new(tab_data)
        .block(tab_block)
        .style(Style::default().white())
        .highlight_style(Style::default().yellow())
        .select(get_tab_index(app.active_tab.clone()))
        .divider(symbols::DOT);

    frame.render_widget(tabs, chunks[1]);

    if app.active_tab == Tab::Apps {
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

        let list_items_slice: Vec<ListItem> = list_items.clone();

        let list = List::new(list_items_slice);

        let mut scroll_view = ScrollView::new(Size::new(
            get_search_area().width - 1,
            get_search_area().height + 1,
        ));

        scroll_view.render_widget(list, chunks[2]);
        scroll_view.render(chunks[2], frame.buffer_mut(), &mut app.scroll);
    } else if app.active_tab == Tab::Messages {
        let messages = Paragraph::new(app.messages.join("\n"));
        frame.render_widget(messages, chunks[2]);
    }
}
