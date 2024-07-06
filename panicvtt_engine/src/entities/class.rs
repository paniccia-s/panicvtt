use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{assets::asset::Asset, mechanics::dice::Dice};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Class {
    uuid: Uuid,
    name: String, 
    hit_die: Dice
}

impl Class {

    pub fn new(name: String, hit_die: Dice) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name, 
            hit_die
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_hit_die(&self) -> Dice {
        self.hit_die
    }
}

impl Asset for Class {
    fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }
    
    fn get_owning_campaign(&self) -> Option<u128> {
        todo!()
    }
}

impl Default for Class {
    fn default() -> Self {
        Self { 
            uuid: Uuid::nil(), 
            name: String::new(),
            hit_die: Dice::D4 
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn getters() {
        let class = Class::new(String::from("Nick Mason"), Dice::D8);
        
        assert_eq!(String::from("Nick Mason"), class.get_name());
        assert_eq!(Dice::D8, class.get_hit_die());
    }
}