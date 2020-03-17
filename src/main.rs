use mongodb::{options::ClientOptions, Client};
use std::env;
use std::process;

fn main() {
    let url = "url";
    let mut client_options = ClientOptions::parse(url).unwrap();

    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).unwrap();

    for db_name in client.list_database_names(None).unwrap() {
        println!("{}", db_name);
    }
}
