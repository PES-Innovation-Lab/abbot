use mongodb::{Collection, Database};

use crate::{config::app_config::AppConfig, types::entity::Panel};


pub struct PanelsCollection {
    pub collection: Collection<Panel>
}

impl PanelsCollection {
    pub fn init(db: &Database, config: &AppConfig) -> PanelsCollection {
        Self{
            collection: db.collection(&config.app.database.collections.panels)
        }
    }
}
