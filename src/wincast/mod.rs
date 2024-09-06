use std::process::Command;

use anyhow::Result;
use sqlite::{Connection, Value};
use walkdir::WalkDir;

pub mod searchresponse;
pub mod searchresults;

use searchresponse::SearchResponse;
use searchresults::SearchResults;

fn insert_db(db: &Connection, data: &SearchResults) {
    db.prepare("INSERT INTO data VALUES (:name, :path, :icon, :type)")
        .unwrap()
        .bind::<&[(_, Value)]>(
            &[
                (":name", data.name.clone().into()),
                (":path", data.path.clone().into()),
                (":icon", data.icon.clone().into()),
                (":type", data.search_type.clone().into()),
            ][..],
        )
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

pub fn index_apps() {
    let db = sqlite::open("./db.sqlite").unwrap();

    let app_paths = get_app_paths();

    db.execute("DROP TABLE IF EXISTS data").unwrap();
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

            insert_db(&db, &data);
        }
    }

    let mut excluded_paths = get_app_paths();
    excluded_paths.push(String::from(
        "C:\\OEM\\Preload\\Autorun\\GUI\\Acer User's Manual",
    ));

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

    for path in drive_labels {
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            let file_path = file.path().display().to_string();
            if excluded_paths.contains(&file_path) {
                continue;
            }
            let data = SearchResults {
                name: file.file_name().to_string_lossy().to_string(),
                path: file_path.to_string(),
                icon: String::new(),
                search_type: String::from("file"),
            };

            insert_db(&db, &data);
        }
    }
}

pub fn search(query: &str) -> Result<SearchResponse> {
    let db = sqlite::open("./db.sqlite").unwrap();

    // List Installed Apps
    let apps = list_installed_apps(query, &db)?;

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
