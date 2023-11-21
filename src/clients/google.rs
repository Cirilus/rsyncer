use google_drive3 as drive3;
use crate::clients::types::Sync;
use drive3::hyper;
use drive3::hyper_rustls::HttpsConnector;
use google_drive3::{hyper_rustls, oauth2};
use crate::clients::config::Config;
use failure::{err_msg, Error};
use tokio::runtime::Runtime;

type DriveHub = drive3::api::DriveHub<HttpsConnector<hyper::client::HttpConnector>>;

pub struct DriverClient {
    pub hub: DriveHub,
}


impl DriverClient {
    fn new(config: &Config) -> Self {
        let mut secret: oauth2::ApplicationSecret = oauth2::ApplicationSecret::default();
        secret.client_secret = config.client_secret?;


        DriverClient{
            hub: DriverClient::create_hub(config).unwrap(),
        }

    }

    fn auth(config: &Config) -> Result<
        oauth2::authenticator::Authenticator<HttpsConnector<hyper::client::HttpConnector>>,
        Error>{
        let secret: oauth2::ConsoleApplicationSecret =
            serde_json::from_str(config.client_secret())?;
        let secret = secret
            .installed
            .ok_or_else(|| err_msg("ConsoleApplicationSecret.installed is None"))?;

        let rt = Runtime::new().unwrap();
        let auth = rt.block_on(
            oauth2::InstalledFlowAuthenticator::builder(
                secret,
                if config.authorize_using_code() {
                    oauth2::InstalledFlowReturnMethod::Interactive
                } else {
                    oauth2::InstalledFlowReturnMethod::HTTPPortRedirect(8081)
                },
            )
                .persist_tokens_to_disk(config.token_file())
                .build(),
        )?;
        Ok(auth)
    }

    fn create_hub(config: &Config) -> Result<DriveHub, Error> {
        let auth = Self::auth(config)?;

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




