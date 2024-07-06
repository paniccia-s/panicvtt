use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::assets::asset::Asset;

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    uuid: Uuid,
    name: String, 
    speed: u8, 
}

impl Race {
    pub fn new(name: String, speed: u8) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name, 
            speed
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name 
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }
}

impl Asset for Race {
    fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }
    
    fn get_owning_campaign(&self) -> Option<u128> {
        todo!()
    }
}

impl Default for Race {
    fn default() -> Self {
        Self { 
            uuid: Uuid::nil(), 
            name: String::new(), 
            speed: 0 
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn getters() {
        let race = Race::new(String::from("Syd Barrett"), 123);

        assert_eq!(String::from("Syd Barrett"), race.get_name());
        assert_eq!(123, race.get_speed());
    }
}