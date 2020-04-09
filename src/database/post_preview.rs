use crate::data_model::post_review;
use mongodb::{options::ClientOptions, Client};

pub fn save_database(collection_name: &str, review_data: &post_review::ReviewData) {
    let mut client_options =
        ClientOptions::parse("mongodb://root:example@localhost:27017").unwrap();

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).unwrap();

    let db = client.database("code-review-manager");

    let collection = db.collection(collection_name);

    if let bson::Bson::Document(document) = bson::to_bson(&review_data).unwrap() {
        collection.insert_one(document, None).unwrap();
    }
}
