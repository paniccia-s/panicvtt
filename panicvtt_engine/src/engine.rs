use std::path::Path;

use crate::{assets::asset_manager::AssetManager, campaigns::campaign::Campaign, entities::{abilities::AbilityScores, entity::Entity}, mechanics::dice::Rng};

pub struct Engine {   
    asset_manager: AssetManager,
    rng: Rng,
}

impl Engine {
    pub fn new(rng: Rng, asset_root: &Path) -> Self {
        Self {
            asset_manager: AssetManager::new(asset_root).unwrap(),  // For now, panic if something goes wrong
            rng
        }
    }

    pub fn new_campaign(&mut self, campaign_name: String, campaign_description: String) -> &Campaign {
        // Create a new campaign through the asset manager - if this fails a Uuid invariant is violated and we cannot continue
        self.asset_manager.create_campaign(campaign_name, campaign_description).unwrap()
    }

    pub fn new_entity(&mut self, builder: EntityBuilder) -> &Entity {
        // Construct the entity
        let (name, class, race, abilities) = builder.build();
        self.asset_manager.create_entity(name, class, race, abilities, &mut self.rng).unwrap()
    }

    // pub fn delete_entity(&mut self, uuid: EntityID) -> Option<Entity> {
    //     self.entities.remove(&uuid) 
    // }

    // pub fn list_entities(&self) -> Vec<&Entity> {
    //     self.entities.values().collect()
    // }

    // pub fn get_ability_score(&self, uuid: EntityID, ability: Ability) -> Option<AbilityScoreIntType>  {
    //     Some(self.entities.get(&uuid)?.get_ability_score(ability))
    // }

    // pub fn get_ability_scores(&self, uuid: EntityID) -> Option<&AbilityScores> {
    //     Some(self.entities.get(&uuid)?.get_ability_scores())
    // }
}

pub struct EntityBuilder {
    name: String, 
    class: Option<u128>,
    race: Option<u128>,
    abilities: Option<AbilityScores>
}

impl EntityBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name, 
            class: None, race: None, 
            abilities: None
        }
    }

    pub fn with_class(mut self, class: u128) -> Self {
        self.class = Some(class); 
        self
    }

    pub fn with_race(mut self, race: u128) -> Self {
        self.race = Some(race);
        self
    }

    pub fn with_abilities(mut self, abilities: AbilityScores) -> Self {
        self.abilities = Some(abilities);
        self
    }

    fn build(self) -> (String, u128, u128, AbilityScores) {
        (
            self.name,
            self.class.unwrap_or(AssetManager::DEFAULT_CLASS_UUID),
            self.race.unwrap_or(AssetManager::DEFAULT_RACE_UUID),
            self.abilities.unwrap_or(AbilityScores::from_defaults()), 
        )
    } 
}

#[cfg(test)]
pub mod tests {
    use std::path::Path;

    use crate::mechanics::dice::Rng;

    use super::Engine;

    #[test]
    pub fn new() {
        let rng = Rng::new(0, 1);
        let engine = Engine::new(rng, Path::new(""));
    }
}