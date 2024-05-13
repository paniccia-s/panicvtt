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
        Self::from_ability_scores(name, AbilityScores::from_defaults())
    }
    
    pub fn from_ability_scores(name: String, abilities: AbilityScores) -> Self {
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
        write!(f, "Entity {} (uuid ...{})", self.name, &uuid_str[uuid_str.len() - 6..])
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

        assert_eq!(entity.name, name_raw);
        assert_eq!(entity.abilities, AbilityScores::from_defaults());
    }

    #[test]
    fn entity_from_ability_scores() {
        let name_raw = "Rick Wright";
        let name = String::from(name_raw);
        let abilities = AbilityScores::new(1, 2, 3, 4, 5, 6);
        let entity = Entity::from_ability_scores(name, abilities.clone());

        assert_eq!(entity.name, name_raw);
        assert_eq!(entity.abilities, abilities);
    }

    #[test]
    fn entity_getters() {
        let name_raw = "Nick Mason";
        let name = String::from(name_raw);
        let abilities = AbilityScores::new(20, 19, 18, 17, 16, 15);
        let entity = Entity::from_ability_scores(name, abilities.clone());

        assert_eq!(entity.get_name(), entity.name);
        assert_eq!(entity.get_uuid(), entity.uuid.as_u128());

        assert_eq!(entity.get_ability_score(Ability::Strength), abilities.get_ability_score(Ability::Strength));
        assert_eq!(entity.get_ability_score(Ability::Dexterity), abilities.get_ability_score(Ability::Dexterity));
        assert_eq!(entity.get_ability_score(Ability::Constitution), abilities.get_ability_score(Ability::Constitution));
        assert_eq!(entity.get_ability_score(Ability::Intelligence), abilities.get_ability_score(Ability::Intelligence));
        assert_eq!(entity.get_ability_score(Ability::Wisdom), abilities.get_ability_score(Ability::Wisdom));
        assert_eq!(entity.get_ability_score(Ability::Charisma), abilities.get_ability_score(Ability::Charisma));
       
        assert_eq!(*entity.get_ability_scores(), abilities);
    }
}
