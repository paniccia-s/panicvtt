use std::{collections::HashMap, fs, io::Error, path::Path};

use uuid::Uuid;

use crate::entities::{class::Class, race::Race};

use super::{asset::Asset, asset_serde::AssetSerde};

pub struct AssetManager {
    classes: HashMap<u128, Class>,
    races: HashMap<u128, Race>
}

impl AssetManager { 

    fn parse_asset<T>(asset_dir: &Path) -> Result<HashMap<u128, T>, Error> where T : Asset {
        let mut map: HashMap<u128, T> = HashMap::new();

        // Iterate over everything in the directory 
        for obj in fs::read_dir(asset_dir)? {
            let obj = obj?;
            let path = obj.path();
            let m = path.extension();
            match m {
                Some(e) => println!("some {}", e.to_str().unwrap_or_default()),
                None => println!("none"),
            }

            // Recur into directories and aggregate everything found 
            if path.is_dir() {
                let subdir = Self::parse_asset(&path)?;
                map.extend(subdir);
            } else if path.extension().unwrap_or_default() == "panic" {
                // Attempt to parse this asset and quit if we can't (for now)
                let asset: T = AssetSerde::deserialize_asset(&path)?;
                map.insert(asset.get_uuid(), asset);
            }
        }

        Ok(map)
    }

    pub fn new(asset_root: &Path) -> Result<Self, Error> {
        let default_class = Class::default();
        let default_race = Race::default();

        let mut classes: HashMap<u128, Class> = HashMap::from([(default_class.get_uuid(), default_class)]);
        let mut races: HashMap<u128, Race> = HashMap::from([(default_race.get_uuid(), default_race)]);

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
                } // Ignore directories that don't match
            }
        }

        Ok(Self {
            classes,
            races
        })
    }

    // Most tests don't need assets - no need to load anything in this case 
    #[cfg(test)]
    pub fn from_test_config() -> Self {

        use crate::mechanics::dice::Dice;

        let c = Class::new(String::from("Testing Class"), Dice::D12);
        let r = Race::new(String::from("Testing Race"), 123);
        
        Self {
            classes: HashMap::from([(c.get_uuid(), c)]), 
            races: HashMap::from([(r.get_uuid(), r)]),
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

    pub fn get_class(&self, uuid: u128) -> Option<&Class> {
        self.classes.get(&uuid)
    }

    pub fn get_race(&self, uuid: u128) -> Option<&Race> {
        self.races.get(&uuid)
    }

    pub fn get_default_class(&self) -> &Class {
        self.classes.get(&Uuid::nil().as_u128()).unwrap()
    }
    pub fn get_default_race(&self) -> &Race {
        self.races.get(&Uuid::nil().as_u128()).unwrap()
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

        // We loaded one class and one race, and there is one default for each - verify each 
        assert_eq!(am.classes.len(), 2);

        let c = am.classes.get(&0x00000000111122223333444444444444u128).unwrap();
        assert_eq!(c.get_name(), String::from("Class Name"));
        assert_eq!(c.get_hit_die(), Dice::D20);

        let default_class = am.classes.get(&Uuid::nil().as_u128()).unwrap();
        assert_eq!(default_class.get_name(), String::new());
        assert_eq!(default_class.get_hit_die(), Dice::D4);

        assert_eq!(am.races.len(), 2);

        let r = am.races.get(&0xaaaaaaaabbbbccccddddeeeeeeeeeeeeu128).unwrap();
        assert_eq!(r.get_name(), String::from("Race Name"));
        assert_eq!(r.get_speed(), 123);

        let default_race = am.races.get(&Uuid::nil().as_u128()).unwrap();
        assert_eq!(default_race.get_name(), String::new());
        assert_eq!(default_race.get_speed(), 0);
    }
}
