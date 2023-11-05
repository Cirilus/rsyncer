mod clients;

use std::path::PathBuf;
use clients::file;
use clients::types;
use crate::clients::file::FileClient;
use crate::clients::types::Sync;
use std::iter::FromIterator;


fn main() {
    let file_client = file::FileClient::new(file::FileConfig{root: PathBuf::from("/") });

    let list_files = file_client.get_list_file(String::from("")).unwrap();

    print!("{:?}", list_files);
}
