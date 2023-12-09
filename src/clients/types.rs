use std::path::PathBuf;
use thiserror::Error;

#[async_trait::async_trait]
pub trait Syncer {
    async fn upload_file(&self, input_path: PathBuf, output_path: PathBuf);
    async fn upload_directory(&self, input_path: PathBuf, output_path: PathBuf);
    async fn download_file(&self, input_path: PathBuf, output_path: PathBuf);
    async fn download_directory(&self, input_path: PathBuf, output_path: PathBuf);
    async fn check_hash_sum(&self, path: PathBuf) -> Result<String, SyncError>;
    async fn get_list_files(&self, path: PathBuf) -> Result<Vec<String>, SyncError>;
    async fn get_file(&self, path: PathBuf) -> Result<String, SyncError>;
}

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Err getting list, err={0}")]
    ErrListFile(String),

    #[error("Failed convert secret to json")]
    ErrAuth(String),
}