use std::io::Result;

pub trait Sync<T>{
    fn new(config: T) -> Self;
    fn load_file(&self, file_path: String) -> Result<()>;
    fn download_file(&self, file_path: String) -> Result<()>;
    fn get_hash_sum(&self, file_path: String) -> Result<String>;
    fn get_list_file(&self, file_path: String) -> Result<Vec<String>>;
}