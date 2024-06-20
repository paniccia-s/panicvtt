use std::{fs::File, io::Error, path::Path};

use serde::de::DeserializeOwned;



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
}