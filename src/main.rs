use std::io;
use sqlite::Connection;
use prettytable::{color, format, row, Attr, Cell, Row, Table};
use clap::Parser;
use colored::*;

pub mod wincast;

use wincast::{searchresponse::SearchResponse, searchresults::SearchResults};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    query: String,

    #[arg(short, long, default_value_t = false)]
    index: bool,
}

fn main() {
    let db = sqlite::open("./db.sqlite").unwrap();

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
}

fn index(db: &Connection) {
    println!("Indexing apps...");

    wincast::index_apps(&db);

    println!("Indexing complete");
}

fn print_search_results(results: SearchResponse) -> Vec<SearchResults> {
    println!("Matches found: {}/{}", results.matches, results.total);
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new(' ', ' ', ' ', ' '),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    table.set_titles(Row::new(vec![
        Cell::new("ID")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("NAME")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("TYPE")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
        Cell::new("PATH")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(color::GREEN)),
    ]));
    for (i, app) in results.search_results.iter().enumerate() {
        table.add_row(row![i + 1, app.name, app.search_type, app.path]);
    }
    table.printstd();

    return results.search_results;
}

fn launch(files: Vec<SearchResults>) {
    println!("Enter ID to launch: ");
    let input = input();
    let id: usize = input.parse().unwrap();

    if id > files.len() {
        println!("Invalid ID");
        return;
    }

    let file = files.get(id - 1).unwrap();

    let _ = opener::open(file.path.clone());
}

fn input() -> String {
    let mut input = String::new();

    let input_res = io::stdin().read_line(&mut input);

    let mut result = String::new();

    match input_res {
        Ok(_) => result = String::from(input.trim()),
        Err(e) => println!("Error in input: {e}"),
    }

    return result;
}
