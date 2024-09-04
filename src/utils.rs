use std::io;

use prettytable::{color, format, row, Attr, Cell, Row, Table};

use crate::wincast::{self, searchresponse::SearchResponse, searchresults::SearchResults};

pub fn index() {
    println!("Indexing apps...");

    wincast::index_apps();

    println!("Indexing complete");
}

pub fn print_search_results(results: SearchResponse) -> Vec<SearchResults> {
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

pub fn launch(files: Vec<SearchResults>) {
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

pub fn input() -> String {
    let mut input = String::new();

    let input_res = io::stdin().read_line(&mut input);

    let mut result = String::new();

    match input_res {
        Ok(_) => result = String::from(input.trim()),
        Err(e) => println!("Error in input: {e}"),
    }

    return result;
}

pub fn greet() -> &'static str {
    return "
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓███████▓▒░   ░▒▓██████▓▒░   ░▒▓██████▓▒░   ░▒▓███████▓▒░ ░▒▓████████▓▒░ 
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░     
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░           ░▒▓█▓▒░     
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓████████▓▒░  ░▒▓██████▓▒░     ░▒▓█▓▒░     
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░        ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░     
▒▓█▓▒░░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░        ░▒▓█▓▒░    ░▒▓█▓▒░     
░▒▓█████████████▓▒░  ░▒▓█▓▒░ ░▒▓█▓▒░░▒▓█▓▒░  ░▒▓██████▓▒░  ░▒▓█▓▒░░▒▓█▓▒░ ░▒▓███████▓▒░     ░▒▓█▓▒░     
    ";
}
