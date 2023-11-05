use std::fs::File;
use std::io::{Result};
use std::{fs, io};
use std::path::{PathBuf};
use crate::clients::types::Sync;
use sha2::{Sha256, Digest};

pub struct FileClient {
    root: PathBuf,
}


pub struct FileConfig{
    pub root: PathBuf,
}

impl Sync<FileConfig> for FileClient {
    fn new(config: FileConfig) -> Self {
        FileClient{ root: config.root }
    }

    fn load_file(&self, file_path: String) -> Result<()> {
        todo!()
    }

    fn download_file(&self, file_path: String) -> Result<()> {
        todo!()
    }

    fn get_hash_sum(&self, file_path: String) -> Result<String> {
        let mut path = self.root.clone();
        path.push(file_path);

        let mut file = File::open(path)?;

        let mut hasher = Sha256::new();

        let bytes_written = io::copy(&mut file, &mut hasher)?;
        let hash_bytes = hasher.finalize();
        Ok(hex::encode(hash_bytes))
    }

    fn get_list_file(&self, file_path: String) -> Result<Vec<String>> {
        let mut path = self.root.clone();

        path.push(file_path);

        let files = fs::read_dir(path)?;

        let result: Vec<String> = files.
            filter_map(|entry| {
                entry.ok().and_then(|e| e.file_name().into_string().ok())
            })
            .collect();

        Ok(result)
    }
}