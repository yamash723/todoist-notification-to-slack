use std::io::Read;
use reqwest;
use serde_json;
use url::Url;

use types;
use envs;

pub fn render_projects() -> Vec<types::ProjectStruct> {
    let resource_type = format!("[\"{}\"]", "projects");
    let sync_data = execute_render(&resource_type);
    sync_data.projects.unwrap()
}

pub fn render_items() -> Vec<types::ItemStruct> {
    let resource_type = format!("[\"{}\"]", "items");
    let sync_data = execute_render(&resource_type);
    sync_data.items.unwrap()
}

fn execute_render(resource_type: &str) -> types::SyncStrust {
    let auth_token = envs::todoist_api_token();
    let params = [
        ("sync_token", "*"),
        ("resource_types", resource_type),
        ("token", &auth_token)
    ];

    let url = Url::parse_with_params("https://todoist.com/api/v7/sync", &params).unwrap();
    let client = reqwest::Client::new().unwrap();
    let mut responce = client.get(url.as_str()).send().unwrap();
    
    if responce.status().is_success() == false {
        panic!("can't get responce from todoist api");
    }

    let mut content = String::new();
    responce.read_to_string(&mut content).expect("Failed");

    let sync_data: types::SyncStrust = serde_json::from_str(&content).unwrap();
    sync_data
}