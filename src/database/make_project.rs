use crate::data_model::make_project;
use mongodb::{options::ClientOptions, Client};

pub fn make_project(project_data: &make_project::ProjectData) {
    let mut client_options =
        ClientOptions::parse("mongodb://root:example@localhost:27017").unwrap();

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).unwrap();

    let db = client.database("code-review-manager");

    let result = db.create_collection(&project_data.project_name, None);
    panic!(result);
}
