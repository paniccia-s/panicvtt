use std::{fs::File, io::{Error, ErrorKind}, path::Path};

use serde::de::DeserializeOwned;

use crate::entities::entity::{Entity, EntitySerde};

use super::asset_manager::AssetManager;



pub(crate) struct AssetSerde {

}

impl AssetSerde {
    pub fn deserialize_asset<T>(path: &Path) -> Result<T, Error> where T : DeserializeOwned {
        let f = File::open(path)?;
        match serde_yaml::from_reader::<File, T>(f) {
            Ok(e) => Ok(e),
            Err(e) => Err(Error::new(std::io::ErrorKind::InvalidData, e))
        }
    }

    pub fn serialize_entity(entity: Entity, path: &Path) -> Result<(), Error> {
        // Serializing is easy - just convert the entity to its serializable form and go ahead
        let e = entity.to_serde();
        let f = File::create(path)?;
        match serde_yaml::to_writer(f, &e) {
            Ok(()) => Ok(()), 
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
        }
    }

    pub fn deserialize_entity<'d>(path: &Path, assets: &'d AssetManager) -> Result<Entity<'d>, Error> {
        // Serialize the entity's template 
        let f = File::open(path)?;
        let e: EntitySerde = match serde_yaml::from_reader(f) {
            Ok(e) => Ok(e),
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }?;

        // Create the Entity
        match Entity::from_serde(e, assets) {
            Some(entity) => Ok(entity),
            None => Err(Error::new(ErrorKind::InvalidData, "Class or race not found")),
        }
    }
}