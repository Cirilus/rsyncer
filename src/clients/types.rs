use std::path::PathBuf;
use thiserror::Error;

#[async_trait::async_trait]
pub trait Syncer {
    async fn upload_file(&self, input_path: PathBuf, output_path: PathBuf);
    async fn upload_directory(&self, input_path: PathBuf, output_path: PathBuf);
    async fn download_file(&self, input_path: PathBuf, output_path: PathBuf);
    async fn download_directory(&self, input_path: PathBuf, output_path: PathBuf);
    async fn check_hash_sum(&self, path: PathBuf) -> Result<String, SyncError>;
    async fn get_list_files(&self, path: &PathBuf) -> Result<Vec<File>, SyncError>;
    async fn get_file(&self, id: &String) -> Result<File, SyncError>;
}
#[derive(Debug, PartialEq, Clone)]
pub struct File {
    pub id: String,
    pub name: String,
}

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Err getting list, err={0}")]
    ErrListFile(String),

    #[error("Err getting file, err={0}")]
    ErrGettingFile(String),

    #[error("Failed convert secret to json, err={0}")]
    ErrAuth(String),

    #[error("Not found, err = {0}")]
    ErrNotFound(String)
}