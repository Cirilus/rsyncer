use std::fs;

mod clients;
// #[tokio::main]
fn main() {
    let config_str = fs::read_to_string("src/config.toml").expect("Failed to read config");

    let config: clients::config::Config = toml::from_str(&config_str).expect("Failed serialized config");

    let driver_client = clients::google::DriverClient::new(&config).await;
}
