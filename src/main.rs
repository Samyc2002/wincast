use anyhow::Result;
use app::{App, CurrentScreen};
use clap::Parser;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};
use ui::ui;
use utils::launch;

pub mod app;
pub mod ui;
pub mod utils;
pub mod wincast;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    query: String,

    #[arg(short, long, default_value_t = false)]
    index: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Setup Terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    // Set up the Crossterm Backend
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create the App Container
    let mut app = App::new();
    run_app(&mut terminal, &mut app)?;

    // Restore Terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    return Ok(());
}

fn run_app<'a, B: Backend>(terminal: &mut Terminal<B>, app: &'a mut App<'a>) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                // Skip key release events
                continue;
            }

            match app.current_screen {
                CurrentScreen::Search => match key.code {
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    KeyCode::Char(ch) => {
                        app.search_query.push(ch);
                        app.search(app.search_query.clone())?;
                    }
                    KeyCode::Backspace => {
                        app.search_query.pop();
                        app.search(app.search_query.clone())?;
                    }
                    KeyCode::Enter => {
                        if let Some(selected_item) = app.selected_item {
                            launch(selected_item.path.clone());
                        }
                        match &app.selected_id {
                            Some(selected_id) => {
                                launch(
                                    app.search_results
                                        .search_results
                                        .get(*selected_id)
                                        .unwrap()
                                        .path
                                        .clone(),
                                );
                                return Ok(());
                            }
                            None => {}
                        };
                    }
                    KeyCode::Up => {
                        match &app.selected_id {
                            Some(selected_id) => {
                                if *selected_id > 0 {
                                    app.selected_id = Some(selected_id - 1);
                                }
                            }
                            None => {
                                app.selected_id = Some(app.search_results.search_results.len() - 1);
                            }
                        };
                    }
                    KeyCode::Down => {
                        if let Some(selected_id) = &app.selected_id {
                            if *selected_id < app.search_results.search_results.len() - 1 {
                                app.selected_id = Some(selected_id + 1);
                            }
                        } else {
                            app.selected_id = Some(0);
                        }
                    }
                    _ => {}
                },
                CurrentScreen::Home => match key.code {
                    KeyCode::Esc => {
                        return Ok(());
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
