use std::io;
use google_drive::Client;
use crate::clients::types::Sync;
use io::Result;

pub struct  GoogleClient {
    client_id: String,
    client_secret: String,
    redirect_url: String,
    token: String,
    refresh_token: String,
    client: Client,
    connected: bool,
}


pub struct GoogleConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub token: String,
    pub refresh_token: String,
}


impl Sync<GoogleConfig> for GoogleClient {
    fn new(config: GoogleConfig) -> Self {
        let google_drive = Client::new(
            String::from(&config.client_id),
            String::from(&config.client_secret),
            String::from(&config.redirect_url),
            String::from(&config.token),
            String::from(&config.refresh_token),
        );

        let client = GoogleClient{
            client_id: config.client_id,
            client_secret: config.client_secret,
            redirect_url: config.redirect_url,
            token: config.token,
            refresh_token: config.refresh_token,
            client: google_drive,
            connected: false,
        };


        client
    }

    fn load_file(&self, file_path: String) -> Result<()> {
        todo!()
    }

    fn download_file(&self, file_path: String) -> Result<()> {
        todo!()
    }

    fn get_hash_sum(&self, file_path: String) -> Result<String> {
        todo!()
    }

    fn get_list_file(&self, file_path: String) -> Result<Vec<String>> {
        todo!()
    }
}




