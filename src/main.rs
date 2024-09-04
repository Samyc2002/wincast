use anyhow::Result;
use app::App;
use clap::Parser;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{error::Error, io};
use ui::ui;
use utils::{greet, index, input, launch, print_search_results};
use wincast::searchresults::SearchResults;

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
    // enable_raw_mode()?;
    // let mut stderr = io::stderr();
    // execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    // Set up the Crossterm Backend
    // let backend = CrosstermBackend::new(stderr);
    // let mut terminal = Terminal::new(backend)?;

    // Create the App Container
    // let mut app = App::new();
    // run_app(&mut terminal, &mut app)?;

    // Restore Terminal
    // disable_raw_mode()?;
    // execute!(
    //     terminal.backend_mut(),
    //     LeaveAlternateScreen,
    //     DisableMouseCapture
    // )?;
    // terminal.show_cursor()?;
    println!("{}", format!("{}", greet()));
    index();
    let query = input();
    let result = wincast::search(&query[..]);
    let mut query_files: Vec<SearchResults> = Vec::new();
    match result {
        Ok(apps) => {
            query_files = print_search_results(apps);
        }
        Err(e) => println!("Error: {e}"),
    }
    launch(query_files);

    return Ok(());
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
    }
}

/* Deprecared Code (kept for future reference)
let args = Args::parse();

println!("{}", "".red());
println!("{}", "▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓███████▓▒░   ░▒▓██████▓▒░   ░▒▓██████▓▒░   ░▒▓███████▓▒░ ░▒▓████████▓▒░ ".red());
println!("{}", "▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░     ".red());
println!("{}", "▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░     ".red());
println!("{}", "▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓████████▓▒░  ░▒▓██████▓▒░     ░▒▓█▓▒░     ".red());
println!("{}", "▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░     ".red());
println!("{}", "▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░     ".red());
println!("{}", "░▒▓█████████████▓▒░  ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓███████▓▒░     ░▒▓█▓▒░     ".red());
println!("{}", "".red());

if args.index {
    index(&db);
}

if args.query != "" {
    let result = wincast::search(&args.query, &db);
    let mut query_files: Vec<SearchResults> = Vec::new();
    match result {
        Ok(apps) => {
            query_files = print_search_results(apps);
        }
        Err(e) => println!("Error: {e}"),
    }
    launch(query_files);
}
Deprecated Code ends here */
