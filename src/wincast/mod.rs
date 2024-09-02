use anyhow::Result;
use sqlite::Connection;
use walkdir::WalkDir;

pub mod searchresponse;
pub mod searchresults;

use searchresponse::SearchResponse;
use searchresults::SearchResults;

pub fn index_apps(db: &Connection) {
    let app_paths = [
        "C:\\Users\\samy3\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu",
        "C:\\ProgramData\\Microsoft\\Windows\\Start Menu",
    ];

    db.execute(
        "DROP TABLE IF EXISTS data",
    )
    .unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS data (name TEXT, path TEXT, icon TEXT, search_type TEXT)",
    )
    .unwrap();

    for path in app_paths {
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            let file_path = file.path().display().to_string();
            let data = SearchResults {
                name: file.file_name().to_string_lossy().to_string(),
                path: file_path.to_string(),
                icon: String::new(),
                search_type: String::from("app"),
            };

            db.execute(format!(
                "INSERT INTO data VALUES ('{}', '{}', '{}', '{}')",
                data.name, data.path, data.icon, data.search_type
            ))
            .unwrap();
        }
    }
}

pub fn search(query: &str, db: &Connection) -> Result<SearchResponse> {
    // List Installed Apps
    let apps = list_installed_apps(query, db)?;

    return Ok(apps);
}

fn list_installed_apps(query: &str, db: &Connection) -> Result<SearchResponse> {
    let mut result = Vec::new();
    let mut matches = 0;
    let mut total = 0;

    let db_query = format!("SELECT * FROM data");
    let mut files: Vec<SearchResults> = Vec::new();
    for row in db
        .prepare(db_query)
        .unwrap()
        .into_iter()
        .map(|row| row.unwrap())
    {
        let row_result = SearchResults {
            name: String::from(row.read::<&str, _>("name")),
            path: String::from(row.read::<&str, _>("path")),
            icon: String::from(row.read::<&str, _>("icon")),
            search_type: String::from(row.read::<&str, _>("search_type")),
        };

        files.push(row_result);
        total += 1;
    }

    for file in files {
        let file_name = file.path.split("\\").last().unwrap();

        if file_name.to_lowercase().contains(&query.to_lowercase()) {
            matches += 1;
            let file_parts: Vec<&str> = file_name.split(".").collect();
            result.push(SearchResults {
                name: file_parts[0].to_string(),
                path: file.path.to_string(),
                icon: String::new(),
                search_type: String::from("app"),
            });
        }
    }

    let response = SearchResponse {
        search_results: result,
        total,
        matches,
    };

    return Ok(response);
}
