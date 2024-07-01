use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assets::asset::Asset;

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    uuid: Uuid, 
    name: String, 
    entities: Vec<u128>
}

impl Scene {
    pub fn new(name: String) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name,
            entities: Vec::new(),
        }
    }

    pub(crate) fn get_name(&self) -> &str {
        &self.name
    }
} 

impl Asset for Scene {
    fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }
}
