use std::{collections::HashMap, fs, io::Error, path::Path};

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
        let mut classes: Option<HashMap<u128, Class>> = None;
        let mut races: Option<HashMap<u128, Race>> = None;

        // Attempt to open the directory provided
        for obj in fs::read_dir(asset_root)? {
            let obj = obj?;
            let path = obj.path();
            
            // Ignore non-directory objects in the root 
            if path.is_dir() {
                // Determine which asset folder we're about to load 
                let dir_name = path.file_name().unwrap_or_default();
                if dir_name == "classes" {
                    classes = Some(Self::parse_asset(&path)?);
                } else if dir_name == "races" {
                    races = Some(Self::parse_asset(&path)?);
                } // Ignore directories that don't match
            }
        }

        Ok(Self {
            classes: classes.unwrap_or_default(),
            races: races.unwrap_or_default(),
        })
    }

    // Most tests don't need assets - no need to load anything in this case 
    #[cfg(test)]
    pub fn from_no_assets() -> Self {
        Self {
            classes: Default::default(), 
            races: Default::default()
        }
    }
}


#[cfg(test)]
pub mod tests {
    use crate::mechanics::dice::Dice;

    use super::*;

    #[test]
    pub fn happy_path() {
        // Read assets from the test assets directory
        let test_asset_root = Path::new("test/assets");
        assert!(test_asset_root.is_dir());
        
        // Load the directory into the asset manager 
        let am = AssetManager::new(test_asset_root).unwrap();

        // We loaded one class and one race - verify each 
        assert_eq!(am.classes.len(), 1);

        let c = am.classes.get(&0x00000000111122223333444444444444u128).unwrap();
        assert_eq!(c.get_name(), String::from("Class Name"));
        assert_eq!(c.get_hit_die(), Dice::D20);

        assert_eq!(am.races.len(), 1);

        let r = am.races.get(&0xaaaaaaaabbbbccccddddeeeeeeeeeeeeu128).unwrap();
        assert_eq!(r.get_name(), String::from("Race Name"));
        assert_eq!(r.get_speed(), 123);
    }
}
