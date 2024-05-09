use std::fmt::Display;

use uuid::Uuid;

use super::entity::format_entity_data;


pub struct EntityView {
    pub uuid: Uuid, 
    pub name: String
} 

impl EntityView {
    
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }

}

impl Display for EntityView {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format_entity_data(f, &self.uuid, &self.name)
    }
}
