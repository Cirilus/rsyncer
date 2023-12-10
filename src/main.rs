use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::clients::types::{Syncer};

mod clients;

#[tokio::main]
async fn main() {
    let config_str = fs::read_to_string("src/config.toml").expect("Failed to read config");

    let config: clients::config::Config = toml::from_str(&config_str).expect("Failed serialized config");

    let driver_client = clients::google::DriverClient::new(&config).await;

    let  mut path = PathBuf::new();

    path.push("");

    let text = driver_client.get_list_files(&path).await.unwrap();

    let file = driver_client.get_file(&text[0].id).await;

    print!("{:?}", file);
}
