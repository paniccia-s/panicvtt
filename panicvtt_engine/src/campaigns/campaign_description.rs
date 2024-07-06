use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CampaignDescription {
    uuid: Uuid, 
    name: String, 
    path: String, 
    description: String,  
}

impl CampaignDescription {

    pub fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_description(&self) -> &str {
        &self.description
    }
}