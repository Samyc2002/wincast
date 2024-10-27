use anyhow::Result;

pub mod searchresponse;
pub mod searchresults;

use searchresponse::SearchResponse;
use searchresults::SearchResults;
use walkdir::WalkDir;

fn get_app_paths() -> Vec<String> {
    return vec![
        String::from(format!(
            "C:\\Users\\{}\\AppData\\Roaming\\Microsoft\\Windows\\Start Menu",
            whoami::username()
        )),
        String::from("C:\\ProgramData\\Microsoft\\Windows\\Start Menu"),
    ];
}

pub fn index_apps() -> SearchResponse {
    let app_paths = get_app_paths();

    let mut app_count = 0;
    let mut results = Vec::new();

    for path in app_paths {
        for file in WalkDir::new(path).into_iter().filter_map(|file| file.ok()) {
            let file_path = file.path().display().to_string();
            let data = SearchResults {
                name: file.file_name().to_string_lossy().to_string(),
                path: file_path.to_string(),
                icon: String::new(),
                search_type: String::from("app"),
            };

            results.push(data);

            app_count += 1;
        }
    }

    return SearchResponse {
        search_results: results.clone(),
        total: results.len() as i32,
        matches: app_count,
    };
}

pub fn search(query: &str) -> Result<SearchResponse> {
    // List Installed Apps
    let apps = list_results(query, "app")?;
    // List Installed Files
    // let files = list_results(query, &db, "file")?;

    return Ok(apps);
}

fn list_results(query: &str, _result_type: &str) -> Result<SearchResponse> {
    let mut result = Vec::new();
    let mut matches = 0;
    let mut total = 0;

    let mut files: Vec<SearchResults> = Vec::new();
    let all_apps = index_apps();
    for app in all_apps.search_results {
        let row_result = app.clone();

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
