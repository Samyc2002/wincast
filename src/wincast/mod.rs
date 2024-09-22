use std::process::Command;

use anyhow::Result;
use sqlite::Connection;
use walkdir::WalkDir;

pub mod searchresponse;
pub mod searchresults;

use searchresponse::SearchResponse;
use searchresults::SearchResults;

fn insert_db(db: &Connection, data: &SearchResults, insert_type: &str) {
    db.execute(format!(
        "INSERT INTO {insert_type}_data VALUES ('{}', '{}', '{}', '{}')",
        data.name.clone().replace("'", "\""),
        data.path.clone().replace("'", "\""),
        data.icon.clone().replace("'", "\""),
        data.search_type.clone()
    ))
    .unwrap();
}

fn get_app_paths() -> Vec<String> {
    return vec![
        String::from(format!(
            "C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu",
            whoami::username()
        )),
        String::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu"),
    ];
}

pub fn add_file_path(file_path: &str, label: Option<&str>) {
    let db = sqlite::open("./db.sqlite").unwrap();

    db.execute("CREATE TABLE IF NOT EXISTS locations (name TEXT, path TEXT)")
        .unwrap();

    match label {
        Some(label) => {
            db.execute(format!(
                "INSERT INTO locations VALUES ('{}', '{}')",
                label, file_path
            ))
            .unwrap();
        }
        None => {
            db.execute(format!(
                "INSERT INTO locations VALUES ('{}', '{}')",
                file_path, file_path
            ))
            .unwrap();
        }
    }
}

pub fn index_apps() -> u16 {
    let db = sqlite::open("./db.sqlite").unwrap();

    let app_paths = get_app_paths();

    db.execute("DROP TABLE IF EXISTS app_data").unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS app_data (name TEXT, path TEXT, icon TEXT, search_type TEXT)",
    )
    .unwrap();

    let mut app_count = 0;

    for path in app_paths {
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            let file_path = file.path().display().to_string();
            let data = SearchResults {
                name: file.file_name().to_string_lossy().to_string(),
                path: file_path.to_string(),
                icon: String::new(),
                search_type: String::from("app"),
            };

            insert_db(&db, &data, "app");

            app_count += 1;
        }
    }

    return app_count;
}

pub fn index_files() -> usize {
    let db = sqlite::open("./db.sqlite").unwrap();

    db.execute("DROP TABLE IF EXISTS file_data").unwrap();
    db.execute(
        "CREATE TABLE IF NOT EXISTS file_data (name TEXT, path TEXT, icon TEXT, search_type TEXT)",
    )
    .unwrap();

    let excluded_paths = get_app_paths();

    let drive_output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "wmic logicaldisk get caption"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("echo hello")
            .output()
            .expect("failed to execute process")
    };

    let drives = drive_output.stdout;
    let drives = String::from_utf8(drives).unwrap();
    let drives: Vec<&str> = drives.split("\r\r\n").collect();

    let mut drive_labels = Vec::new();
    for (id, drive) in drives.iter().enumerate() {
        if id > 0 {
            let drive_name = *drive;
            let mut drive_name = String::from(drive_name.trim());
            drive_name.push_str("\\");
            if drive_name.len() > 0 {
                drive_labels.push(drive_name);
            }
        }
    }

    let mut file_count = 0;

    for path in drive_labels {
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            let file_path = file.path().display().to_string();

            // Not scanning apps again
            let mut excluded = false;
            for path in excluded_paths.iter() {
                if file_path.starts_with(path) {
                    excluded = true;
                    break;
                }
            }
            if excluded {
                continue;
            }

            let data = SearchResults {
                name: file.file_name().to_string_lossy().to_string(),
                path: file_path.to_string(),
                icon: String::new(),
                search_type: String::from("file"),
            };

            insert_db(&db, &data, "file");

            file_count += 1;
        }
    }

    return file_count;
}

pub fn search(query: &str) -> Result<SearchResponse> {
    let db = sqlite::open("./db.sqlite").unwrap();

    // List Installed Apps
    let apps = list_results(query, &db, "app")?;
    // List Installed Files
    // let files = list_results(query, &db, "file")?;

    return Ok(apps);
}

fn list_results(query: &str, db: &Connection, result_type: &str) -> Result<SearchResponse> {
    let mut result = Vec::new();
    let mut matches = 0;
    let mut total = 0;

    db.execute(
        format!("CREATE TABLE IF NOT EXISTS {result_type}_data (name TEXT, path TEXT, icon TEXT, search_type TEXT)"),
    )
    .unwrap();
    let db_query = format!("SELECT * FROM {result_type}_data");
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
                icon: file.icon,
                search_type: file.search_type,
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
