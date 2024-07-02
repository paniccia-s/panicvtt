use std::path::Path;

use crate::{assets::asset_manager::AssetManager, campaigns::campaign::Campaign, entities::{abilities::AbilityScores, class::Class, entity::Entity, race::Race}, mechanics::dice::{Dice, Rng}};

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

    pub fn new_class(&mut self, class_name: String, hit_die: Dice) -> &Class {
        // Create a new class through the asset manager - if this fails a Uuid invariant is violated and we cannot continue
        self.asset_manager.create_class(class_name, hit_die).unwrap()
    }

    pub fn new_race(&mut self, race_name: String, speed: u8) -> &Race {
        // Create a new race through the asset manager - if this fails a Uuid invariant is violated and we cannot continue
        self.asset_manager.create_race(race_name, speed).unwrap()
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

    use crate::{assets::{asset::Asset, asset_manager::AssetManager}, engine::EntityBuilder, entities::abilities::AbilityScores, mechanics::dice::{Dice, Rng}};

    use super::Engine;

    #[test]
    pub fn new() {
        let rng = Rng::new(0, 1);
        let mut engine = Engine::new(rng, Path::new("test/assets"));

        // Add a new Campaign and make sure it's there 
        let campaign_id: u128;
        { 
            let campaign = engine.new_campaign(String::from("Campaign name"), String::from("A super thirlling campaign"));
            assert_eq!(campaign.get_name(), String::from("Campaign name"));
            campaign_id = campaign.get_uuid();
        }

        let c = engine.asset_manager.get_campaign(campaign_id).unwrap();
        assert_eq!(c.get_uuid(), campaign_id);

        let entity_id: u128;
        {
            let entity = engine.new_entity(EntityBuilder::new(String::from("Test Entity")));
            assert_eq!(entity.get_name(), String::from("Test Entity"));
            entity_id = entity.get_uuid();
        }

        let e = engine.asset_manager.get_entity(entity_id).unwrap();
        assert_eq!(e.get_uuid(), entity_id);

        let class_id: u128; 
        {
            let class = engine.new_class(String::from("Class 2"), Dice::D10);
            assert_eq!(class.get_name(), String::from("Class 2"));
            class_id = class.get_uuid();
        }

        let c = engine.asset_manager.get_class(class_id).unwrap();
        assert_eq!(c.get_uuid(), class_id);

        let race_id: u128;
        {
            let race = engine.new_race(String::from("Race 2"), 51);
            assert_eq!(race.get_name(), String::from("Race 2"));
            race_id = race.get_uuid();
        }

        let r = engine.asset_manager.get_race(race_id).unwrap();
        assert_eq!(r.get_uuid(), race_id);
    }

    #[test]
    pub fn entity_builder() {
        let rng = Rng::new(0, 1);
        let mut engine = Engine::new(rng, Path::new("test/assets"));

        let entity_id: u128;
        let class_id: u128;
        let race_id: u128;

        {
            let c = engine.asset_manager.get_class(AssetManager::DEFAULT_CLASS_UUID).unwrap().get_uuid();
            let r = engine.asset_manager.get_race(AssetManager::DEFAULT_RACE_UUID).unwrap().get_uuid();
            class_id = c;
            race_id = r;

            let a = AbilityScores::new(1, 6, 11, 16, 21, 26);
            let e = engine.new_entity(EntityBuilder::new(String::from("Builder Entity"))
            .with_class(c)
            .with_race(r)
            .with_abilities(a)
        );
        
            assert_eq!(e.get_name(), "Builder Entity");
            entity_id = e.get_uuid();
        }

        let e = engine.asset_manager.get_entity(entity_id).unwrap();
        assert_eq!(e.get_class(&engine.asset_manager).unwrap().get_uuid(), class_id);
        assert_eq!(e.get_race(&engine.asset_manager).unwrap().get_uuid(), race_id);
        assert_eq!(*e.get_ability_scores(), AbilityScores::new(1, 6, 11, 16, 21, 26));
    }
}