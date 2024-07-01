use std::{collections::HashMap, fs::{self, File}, io::{Error, ErrorKind}, path::Path};

use serde::de::DeserializeOwned;
use uuid::Uuid;

use crate::{campaigns::campaign::Campaign, entities::{abilities::AbilityScores, class::Class, entity::Entity, race::Race}, mechanics::dice::{Dice, Rng}, util::asset_key_error::AssetKeyError};

use super::asset::Asset;

pub struct AssetManager {
    campaigns: HashMap<u128, Campaign>,
    classes: HashMap<u128, Class>,
    entities: HashMap<u128, Entity>,
    races: HashMap<u128, Race>, 
}

macro_rules! create_and_check_dups {
    ($asset:ident, $map:ident, $message:literal) => {{
        let uuid = $asset.get_uuid();
        match $map.insert($asset.get_uuid(), $asset) {
            Some(previous_asset) => {
                // There already exists an Asset with this UUID! 
                let new_asset = $map.get(&uuid).unwrap(); 
                Err(AssetKeyError::new(
                    String::from($message), 
                    previous_asset, new_asset, $map
                ))
            }, 
            None => {
                // All is fine
                Ok($map.get(&uuid).unwrap())
            }
        }
    }}
}

impl AssetManager { 

    pub(crate) const DEFAULT_CLASS_UUID: u128 = 0;
    pub(crate) const DEFAULT_RACE_UUID: u128 = 0;

    fn parse_asset<T>(asset_dir: &Path) -> Result<HashMap<u128, T>, Error> where T : Asset + DeserializeOwned {
        let mut map: HashMap<u128, T> = HashMap::new();

        // Iterate over everything in the directory 
        for obj in fs::read_dir(asset_dir)? {
            let obj = obj?;
            let path = obj.path();

            // Recur into directories and aggregate everything found 
            if path.is_dir() {
                let subdir = Self::parse_asset(&path)?;
                map.extend(subdir);
            } else if path.extension().unwrap_or_default() == "panic" {
                // Attempt to parse this asset and quit if we can't (for now)
                let f = File::open(path)?;
                let asset: T = serde_yaml::from_reader(f).map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
                map.insert(asset.get_uuid(), asset);
            }
        }

        Ok(map)
    }

    //TODO Might want to eventually look for duplicate UUIDs when deserializing... 
    pub fn new(asset_root: &Path) -> Result<Self, Error> {
        let default_class = Class::default();
        let default_race = Race::default();

        let mut classes: HashMap<u128, Class> = HashMap::from([(default_class.get_uuid(), default_class)]);
        let mut races: HashMap<u128, Race> = HashMap::from([(default_race.get_uuid(), default_race)]);
        let mut entities: HashMap<u128, Entity> = HashMap::new();
        let mut campaigns: HashMap<u128, Campaign> = HashMap::new();

        // Attempt to open the directory provided
        for obj in fs::read_dir(asset_root)? {
            let obj = obj?;
            let path = obj.path();
            
            // Ignore non-directory objects in the root 
            if path.is_dir() {
                // Determine which asset folder we're about to load 
                let dir_name = path.file_name().unwrap_or_default();
                if dir_name == "classes" {
                    classes.extend(Self::parse_asset(&path)?);
                } else if dir_name == "races" {
                    races.extend(Self::parse_asset(&path)?);
                } else if dir_name == "entities" {
                    entities.extend(Self::parse_asset(&path)?);
                } else if dir_name == "campaigns" {
                    campaigns.extend(Self::parse_asset(&path)?);
                } // Ignore directories that don't match
            }
        }

        Ok(Self {
            campaigns,
            classes,
            races, 
            entities
        })
    }

    // Most tests don't need assets - no need to load anything in this case 
    #[cfg(test)]
    pub fn from_test_config() -> Self {

        use crate::mechanics::dice::Dice;

        let c = Class::new(String::from("Testing Class"), Dice::D12);
        let r = Race::new(String::from("Testing Race"), 123);
        
        Self {
            campaigns: HashMap::new(),
            classes: HashMap::from([(c.get_uuid(), c)]), 
            races: HashMap::from([(r.get_uuid(), r)]),
            entities: HashMap::new(),
        }
    }

    #[cfg(test)]
    pub fn get_testing_class(&self) -> &Class {
        self.classes.iter().next().unwrap().1
    }

    #[cfg(test)]
    pub fn get_testing_race(&self) -> &Race {
        self.races.iter().next().unwrap().1
    }

    pub fn get_campaign(&self, uuid: u128) -> Option<&Campaign> {
        self.campaigns.get(&uuid)
    }

    pub fn get_class(&self, uuid: u128) -> Option<&Class> {
        self.classes.get(&uuid)
    }

    pub fn get_race(&self, uuid: u128) -> Option<&Race> {
        self.races.get(&uuid)
    }

    pub fn get_entity(&self, uuid: u128) -> Option<&Entity> {
        self.entities.get(&uuid)
    }
    
    pub fn get_default_class(&self) -> &Class {
        self.classes.get(&Uuid::nil().as_u128()).unwrap()
    }
    
    pub fn get_default_race(&self) -> &Race {
        self.races.get(&Uuid::nil().as_u128()).unwrap()
    }



    pub(crate) fn create_campaign(&mut self, campaign_name: String, campaign_description: String) -> Result<&Campaign, AssetKeyError<'_, Campaign>> {
        let c = Campaign::new(campaign_name, campaign_description);
        let campaigns = &mut self.campaigns;

        create_and_check_dups!(c, campaigns, "c")
    }

    pub(crate) fn create_class(&mut self, class_name: String, hit_die: Dice) -> Result<&Class, AssetKeyError<Class>> {
        let c = Class::new(class_name, hit_die);
        let classes = &mut self.classes;

        create_and_check_dups!(c, classes, "c")
    }

    pub(crate) fn create_entity(&mut self, entity_name: String, class: u128, race: u128, abilities: AbilityScores, rng: &mut Rng) -> Result<&Entity, AssetKeyError<Entity>> {
        let e = Entity::new(entity_name, class, race, abilities, self, rng);
        let entities = &mut self.entities;

        create_and_check_dups!(e, entities, "d")
    }

    pub(crate) fn create_race(&mut self, race_name: String, speed: u8) -> Result<&Race, AssetKeyError<Race>> {
        let r = Race::new(race_name, speed);
        let races = &mut self.races; 

        create_and_check_dups!(r, races, "r")
    }

}


#[cfg(test)]
pub mod tests {
    use uuid::Uuid;

    use crate::mechanics::dice::Dice;

    use super::*;

    #[test]
    pub fn happy_path() {
        // Read assets from the test assets directory
        let test_asset_root = Path::new("test/assets");
        assert!(test_asset_root.is_dir());
        
        // Load the directory into the asset manager 
        let am = AssetManager::new(test_asset_root).unwrap();

        // We loaded two classes and two races, and there is one default for each - verify each 
        assert_eq!(am.classes.len(), 3);

        let c1 = am.classes.get(&0x00000000111122223333444444444444u128).unwrap();
        assert_eq!(c1.get_name(), String::from("Class Name"));
        assert_eq!(c1.get_hit_die(), Dice::D20);

        let c2 = am.classes.get(&0x00000000000000000000123456789000u128).unwrap();
        assert_eq!(c2.get_name(), String::from("Nested Class Name"));
        assert_eq!(c2.get_hit_die(), Dice::D100);

        let default_class = am.classes.get(&Uuid::nil().as_u128()).unwrap();
        assert_eq!(default_class.get_name(), String::new());
        assert_eq!(default_class.get_hit_die(), Dice::D4);

        assert_eq!(am.races.len(), 3);

        let r1 = am.races.get(&0xaaaaaaaabbbbccccddddeeeeeeeeeeeeu128).unwrap();
        assert_eq!(r1.get_name(), String::from("Race Name"));
        assert_eq!(r1.get_speed(), 123);

        let r2 = am.races.get(&0x99999999999999999999999999999999u128).unwrap();
        assert_eq!(r2.get_name(), String::from("Another Race Name! 😀"));
        assert_eq!(r2.get_speed(), 255);

        let default_race = am.races.get(&Uuid::nil().as_u128()).unwrap();
        assert_eq!(default_race.get_name(), String::new());
        assert_eq!(default_race.get_speed(), 0);

        // We loaded one entity: make sure its class/race are correct 
        let e = am.entities.get(&0xeeeeeeeeddddccccbbbbaaaaaaaaaaaau128).unwrap();
        assert_eq!(e.get_class_name(&am).unwrap(), c1.get_name());
        assert_eq!(e.get_race_name(&am).unwrap(), r1.get_name());
    }

    #[test]
    pub fn default_getters() {
        let test_asset_root = Path::new("test/assets");
        assert!(test_asset_root.is_dir());

        let am = AssetManager::new(test_asset_root).unwrap();

        let c = am.get_default_class();
        let r = am.get_default_race();

        assert_eq!(c.get_name(), String::from(""));
        assert_eq!(c.get_hit_die(), Dice::D4);
        assert_eq!(c.get_uuid(), Uuid::nil().as_u128());

        assert_eq!(r.get_name(), String::from(""));
        assert_eq!(r.get_speed(), 0);
        assert_eq!(r.get_uuid(), Uuid::nil().as_u128());
    }

    #[test]
    pub fn bad_asset_fails_correctly() {
        let am = AssetManager::from_test_config();

        // Try to load some Assets with bad IDs 
        let bad_class = am.get_class(55u128);
        assert!(bad_class.is_none());

        let bad_race = am.get_race(55u128);
        assert!(bad_race.is_none());

        let bad_entity = am.get_entity(55u128);
        assert!(bad_entity.is_none());
    }

    #[test]
    pub fn create_happy_path() {
        let mut am = AssetManager::from_test_config();

        // One of each asset with no errors
        let Ok(campaign) = am.create_campaign(
            String::from("Test Campaign Name"), String::from("Test Campaign Description")) else { panic!() };
        let campaign_uuid = campaign.get_uuid();
        let campaign = am.get_campaign(campaign_uuid).unwrap();

        assert_eq!(campaign.get_uuid(), campaign_uuid);
        assert_eq!(campaign.get_name(), "Test Campaign Name");
        assert_eq!(campaign.get_description(), "Test Campaign Description");

        let Ok(class) = am.create_class(String::from("Test Class Name"), Dice::D20) else { panic!() };
        let class_uuid = class.get_uuid();
        let class = am.get_class(class_uuid).unwrap();

        assert_eq!(class.get_name(), "Test Class Name");
        
        let Ok(race) = am.create_race(String::from("Test Race Name"), 222) else { panic!() };
        let race_uuid = race.get_uuid();
        let race = am.get_race(race_uuid).unwrap();

        assert_eq!(race.get_name(), "Test Race Name");
        
        let mut rng = Rng::new(0, 0);
        let Ok(entity) = am.create_entity(
           String::from("Test Entity Name"), class_uuid, race_uuid, 
           AbilityScores::from_defaults(), &mut rng) else { panic!() };
        let entity_uuid = entity.get_uuid();
        let entity = am.get_entity(entity_uuid).unwrap();

        assert_eq!(entity.get_name(), "Test Entity Name");
    }

}
