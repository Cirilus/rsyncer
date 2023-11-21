use google_drive3::client::serde_with::serde::Deserialize;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    pub debug: Option<bool>,

    pub mount_check: Option<bool>,

    pub cache_statfs_seconds: Option<u64>,

    pub sync_interval: Option<u64>,

    pub mount_options: Option<Vec<String>>,

    pub config_dir: Option<PathBuf>,

    pub session_name: Option<String>,

    pub authorize_using_code: Option<bool>,

    pub rename_identical_files: Option<bool>,

    pub add_extensions_to_special_files: Option<bool>,

    pub skip_trash: Option<bool>,

    pub client_secret: Option<String>,
}

impl Config {
    pub fn debug(&self) -> bool {
        self.debug.unwrap_or(false)
    }

    pub fn sync_interval(&self) -> Duration {
        Duration::from_secs(self.sync_interval.unwrap_or(10))
    }

    pub fn client_secret(&self) -> &String {
        self.client_secret.as_ref().unwrap()
    }
}