use std::fmt::Display;

use uuid::Uuid;

use super::abilities::{Ability, AbilityScoreIntType, AbilityScores};

/// An Entity is an agent within the engine that is able to be unique identified and interacted with. 
pub struct Entity {
    uuid: Uuid,
    name: String, 
    abilities: AbilityScores,
}

impl Entity {
    pub fn new(name: String) -> Entity {
        Self::from_attributes(name, AbilityScores::from_defaults())
    }
    
    pub fn from_attributes(name: String, abilities: AbilityScores) -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name, 
            abilities
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_uuid(&self) -> u128 {
        self.uuid.as_u128()
    }

    pub fn get_ability_score(&self, ability: Ability) -> AbilityScoreIntType {
        self.abilities.get_ability_score(ability)
    }

    pub fn get_ability_scores(&self) -> &AbilityScores {
        &self.abilities
    }
}

impl Display for Entity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uuid_str = self.uuid.as_u128().to_string();
        write!(f, "Entity {} (uuid ...{}) {{{}}}", self.name, &uuid_str[uuid_str.len() - 6..], self.abilities)
    }
}
 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_new() {
        let name_raw = "David Gilmour";
        let name = String::from(name_raw);
        let entity = Entity::new(name);
        assert_eq!(entity.get_name(), name_raw);
    }
}
