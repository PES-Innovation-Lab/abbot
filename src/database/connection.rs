use std::{path::PathBuf, sync::OnceLock};

use mongodb::{options::{AuthMechanism, ClientOptions, Credential, Tls, TlsOptions}, Client};

use crate::config::app_config::AppConfig;
use lazy_static::lazy_static;

use super::{candidates::CandidatesCollection, panels::PanelsCollection};

lazy_static! {
    pub static ref CANDIDATE_DB: OnceLock<CandidatesCollection> = OnceLock::new();
    pub static ref PANELS_DB: OnceLock<PanelsCollection> = OnceLock::new();
}

pub async fn init_client(app_config: &AppConfig) -> Result<(), mongodb::error::Error> {
    let host = &app_config.app.database.atlas_host;
    let db_name = &app_config.app.database.db_name;
    let cert_path = &app_config.app.database.certificate_path;
    let uri = format!("mongodb+srv://{host}/?authSource=%24external&authMechanism=MONGODB-X509&retryWrites=true&w=majority");
    
    let mut client_options = ClientOptions::parse(uri).await?;
    client_options.credential = Some(
        Credential::builder()
            .mechanism(AuthMechanism::MongoDbX509)
            .build(),
    );
    let tls_options = TlsOptions::builder()
        .cert_key_file_path(PathBuf::from(cert_path))
        .build();
    client_options.tls = Some(Tls::Enabled(tls_options));
    let client = Client::with_options(client_options)?;
    let database = client.database(db_name);
    CANDIDATE_DB.get_or_init(|| { CandidatesCollection::init(&database, &app_config) });
    PANELS_DB.get_or_init(|| { PanelsCollection::init(&database, &app_config) });
    Ok(())
}