use std::{fs::File, io::{Error, ErrorKind}, path::Path};

use serde::de::DeserializeOwned;

use super::{asset_manager::AssetManager, reference_serializable::ReferenceSerializable};



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



    pub fn serialize_reference_serializable<'rs, T>(val: T, path: &Path) -> Result<(), Error> where T : ReferenceSerializable<'rs> {
        // Serializing is easy - just convert the T to its serializable form and go ahead 
        let v = val.serialize();
        let f = File::create(path)?;
        match serde_yaml::to_writer(f, &v) {
            Ok(()) => Ok(()), 
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e))
        }
    }

    pub fn deserialize_reference_serializable<'rs, T>(path: &Path, assets: &'rs AssetManager) -> Result<T::Deserialized, Error> where T : ReferenceSerializable<'rs> {
        // Deserialize the entity's template 
        let f = File::open(path)?;
        let v = match serde_yaml::from_reader(f) {
            Ok(v) => Ok(v), 
            Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
        }?;

        // Create the object itself 
        match T::deserialize(v, assets) {
            Some(val) => Ok(val), 
            None => Err(Error::new(ErrorKind::InvalidData, "Uh oh "))
        }
    }

}