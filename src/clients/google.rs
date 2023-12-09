use std::path::PathBuf;
use google_drive3 as drive3;
use drive3::hyper;
use drive3::hyper_rustls::HttpsConnector;
use google_drive3::{hyper_rustls, oauth2};
use crate::clients::config::Config;
use crate::clients::types::{Syncer, SyncError};
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

    async fn get_list_files(&self, path: PathBuf) -> Result<Vec<String>, SyncError> {
        let result = self.hub.files().list().doit().await;
        let _ = return match result {
            Ok(res) => Ok(vec!["Tes1t".to_string()]),
            Err(e) => Err(SyncError::ErrListFile(e.to_string())),
        };
    }

    async fn get_file(&self, path: PathBuf) -> Result<String, SyncError> {
        todo!()
    }
}

