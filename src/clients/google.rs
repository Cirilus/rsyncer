use std::path::PathBuf;
use google_drive3 as drive3;
use drive3::hyper;
use drive3::hyper_rustls::HttpsConnector;
use google_drive3::{hyper_rustls, oauth2};
use thiserror::Error;
use crate::clients::config::Config;
use crate::clients::types::{Syncer, SyncError};
use serde_json::Error as serde_error;
type DriveHub = drive3::api::DriveHub<HttpsConnector<hyper::client::HttpConnector>>;

pub struct DriverClient {
    pub hub: DriveHub,
}

#[derive(Error, Debug)]
enum DriveError {
    #[error("Failed convert secret to json")]
    ErrParseSecretKet(#[from] serde_error),

    #[error("Failed convert secret to json")]
    ErrAuth(#[from] std::io::Error),

    #[error("Failed to installed secret")]
    ErrInstalledSecret()
}

impl DriverClient {
    pub async fn new(config: &Config) -> Self {
        DriverClient{
            hub: DriverClient::create_hub(config).await.unwrap(),
        }

    }

    async fn auth(config: &Config) -> Result<
        oauth2::authenticator::Authenticator<HttpsConnector<hyper::client::HttpConnector>>,
        DriveError>{
        let secret: oauth2::ConsoleApplicationSecret =
            serde_json::from_str(config.client_secret())?;
        let secret = secret
            .installed
            .ok_or_else(DriveError::ErrInstalledSecret)?;

        let auth =
            oauth2::InstalledFlowAuthenticator::builder(
                secret,
                oauth2::InstalledFlowReturnMethod::HTTPRedirect,
            ).build().await?;
        Ok(auth)
    }

    async fn create_hub(config: &Config) -> Result<DriveHub, DriveError> {
        let auth = Self::auth(config).await?;

        Ok(google_drive3::DriveHub::new(
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
    fn upload_file(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    fn upload_directory(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    fn download_file(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    fn download_directory(&self, input_path: PathBuf, output_path: PathBuf) {
        todo!()
    }

    fn check_hash_sum(&self, path: PathBuf) -> Result<String, SyncError> {
        todo!()
    }

    async fn get_list_files(&self, path: PathBuf) -> Result<Vec<String>, SyncError> {
        let result = self.hub.files().list().doit().await;
        let _ = match result {
            Err(e) => Err(SyncError::ErrListFile(e.to_string())),
            Ok(res) => Ok(vec!["Test".to_string()]),
        };
        Ok(vec!["Test".to_string()])
    }

    fn get_file(&self, path: PathBuf) -> Result<String, SyncError> {
        todo!()
    }
}

