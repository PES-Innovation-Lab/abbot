use futures::TryStreamExt;
use mongodb::{error::Error, Collection, Database};

use crate::{config::app_config::AppConfig, types::entity::Candidate};

#[derive(Clone)]
pub struct CandidatesCollection {
    pub collection: Collection<Candidate>
}

impl CandidatesCollection {

    pub fn init(db: &Database, config: &AppConfig) -> Self {
        Self {
            collection: db.collection(&config.app.database.collections.candidates)
        }
    }

    pub async fn get_all_candidates(&self) -> Result<Vec<Candidate>, Error> {
        let candidates: Vec<Candidate> = self.collection
            .find(None, None).await?
            .try_collect().await?;
        Ok(candidates)
    }
}