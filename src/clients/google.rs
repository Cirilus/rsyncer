use std::fmt::Debug;
use std::path::PathBuf;
use google_drive3 as drive3;
use drive3::hyper;
use drive3::hyper_rustls::HttpsConnector;
use google_drive3::{hyper_rustls, oauth2};
use crate::clients::config::Config;
use crate::clients::types::{File, Syncer, SyncError};
use crate::clients::types::SyncError::{ErrGettingFile, ErrNotFound};

type DriveHub = drive3::api::DriveHub<HttpsConnector<hyper::client::HttpConnector>>;

pub struct DriverClient {
    pub hub: DriveHub,
}

impl DriverClient {
    pub async fn new(config: &Config) -> Self {
        DriverClient{
            hub: DriverClient::create_hub(config).await.unwrap(),
        }

    }

    async fn auth(config: &Config) -> Result<
        oauth2::authenticator::Authenticator<HttpsConnector<hyper::client::HttpConnector>>,
        SyncError>{
        let secret: oauth2::ConsoleApplicationSecret = match serde_json::from_str(config.client_secret()){
            Ok(sec) => sec,
            Err(err) => return Err(SyncError::ErrAuth(format!("err convert to json, err = {}", err)))
        };

        let secret = match secret.installed {
            Some(sec) => sec,
            None => return Err(SyncError::ErrAuth("err install secret".to_string()))
        };

        let res =
            oauth2::InstalledFlowAuthenticator::builder(
                secret,
                oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            ).build().await;

        let auth = match res {
            Ok(auth) => auth,
            Err(err) => return Err(SyncError::ErrAuth(format!("err building oauth, err = {}", err)))
        };
        Ok(auth)
    }

    async fn create_hub(config: &Config) -> Result<DriveHub, SyncError> {
        let res = Self::auth(config).await;

        let auth = match res {
            Ok(auth) => auth,
            Err(err) => return Err(SyncError::ErrAuth(format!("auth err: {}", err)))
        };

        Ok(DriveHub::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        ))
    }
}

#[async_trait::async_trait]
impl Syncer for DriverClient {
    async fn upload_file(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    async fn upload_directory(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    async fn download_file(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    async fn download_directory(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    async fn check_hash_sum(&self, path: PathBuf) -> Result<String, SyncError> {
        todo!()
    }

    async fn get_list_files(&self, path: &PathBuf) -> Result<Vec<File>, SyncError> {
        let result = self.hub.files().list().doit().await;
        let resp = match result {
            Ok(res) => res.1,
            Err(e) => return Err(SyncError::ErrListFile(e.to_string())),
        };

        let resp = match resp.files {
            Some(resp) => resp,
            None => return Err(ErrNotFound("there is no file in such path".to_string()))
        };

        let files = resp.iter().
            map(|file| File{
                id: file.id.clone().unwrap_or("".to_string()),
                name: file.name.clone().unwrap_or("".to_string()),
            })
            .collect();
        Ok(files)
    }

    async fn get_file(&self, id: &String) -> Result<File, SyncError> {
        let result = self.hub.files().get(&id).doit().await;

        let resp = match result {
            Ok(resp) => resp.1,
            Err(err) => return Err(ErrGettingFile(err.to_string()))
        };

        Ok(File{
            id: resp.id.clone().unwrap_or("".to_string()),
            name: resp.name.clone().unwrap_or("".to_string()),
        })
    }
}

