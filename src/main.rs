use anyhow::Result;
use app::{App, Tab};
use clap::Parser;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers,
    },
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
use wincast::index_apps;

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

            if key.modifiers.contains(KeyModifiers::CONTROL) {
                // Syncronize/Indexing
                if key.code == KeyCode::Char('s') {
                    app.add_message("Indexing Apps");
                    let app_count = index_apps();
                    app.add_message(format!("Indexed {app_count} apps").as_str());
                    /* Indexing files and folders logic
                        app.add_message("Indexing Files and Folders");
                        let file_count = index_files();
                        app.add_message(
                            format!("Indexed {file_count} files and folders in total").as_str(),
                        );
                    */
                }
                // Switch to Apps
                if key.code == KeyCode::Char('1') {
                    app.active_tab = Tab::Apps;
                }
                // Switch to Messages
                if key.code == KeyCode::Char('2') {
                    app.active_tab = Tab::Messages;
                }
                // Open in google
                if key.code == KeyCode::Char('g') {
                    if app.search_query.len() > 0 {
                        app.add_message(
                            &format!("Searching for {} on Google", app.search_query)[..],
                        );
                        launch(format!(
                            "https://www.google.com/search?q={}",
                            app.search_query.replace(" ", "+")
                        ));
                    }
                }
                // Open in YouTube
                if key.code == KeyCode::Char('y') {
                    if app.search_query.len() > 0 {
                        app.add_message(
                            &format!("Searching for {} on YouTube", app.search_query)[..],
                        );
                        launch(format!(
                            "https://www.youtube.com/results?search_query={}",
                            app.search_query.replace(" ", "+")
                        ));
                    }
                }
                // Open in YouTube Music
                if key.code == KeyCode::Char('m') {
                    if app.search_query.len() > 0 {
                        app.add_message(
                            &format!("Searching for {} on YouTube Music", app.search_query)[..],
                        );
                        launch(format!(
                            "https://music.youtube.com/search?q={}",
                            app.search_query.replace(" ", "+")
                        ));
                    }
                }
                continue;
            }

            match key.code {
                KeyCode::Esc => {
                    return Ok(());
                }
                KeyCode::Char(ch) => {
                    if app.active_tab == Tab::Apps {
                        app.search_query.push(ch);
                        app.search(app.search_query.clone())?;
                    }
                }
                KeyCode::Backspace => {
                    if app.active_tab == Tab::Apps {
                        app.search_query.pop();
                        app.search(app.search_query.clone())?;
                    }
                }
                KeyCode::Enter => {
                    if app.active_tab == Tab::Apps {
                        if let Some(selected_item) = app.selected_item {
                            launch(selected_item.path.clone());
                        }
                        if let Some(selected_id) = &app.selected_id {
                            launch(
                                app.search_results
                                    .search_results
                                    .get(*selected_id)
                                    .unwrap()
                                    .path
                                    .clone(),
                            );
                            return Ok(());
                        };
                    }
                }
                KeyCode::Up => {
                    if app.active_tab == Tab::Apps {
                        if let Some(selected_id) = &app.selected_id {
                            if *selected_id > 0 {
                                app.selected_id = Some(selected_id - 1);
                                app.scroll.scroll_up();
                            }
                        } else {
                            app.selected_id = Some(0);
                        }
                    }
                }
                KeyCode::Down => {
                    if app.active_tab == Tab::Apps {
                        if let Some(selected_id) = &app.selected_id {
                            if *selected_id < app.search_results.search_results.len() - 1 {
                                app.scroll.scroll_down();
                                app.selected_id = Some(selected_id + 1);
                            }
                        } else {
                            app.selected_id = Some(0);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
