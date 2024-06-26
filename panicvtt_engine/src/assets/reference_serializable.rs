
use serde::{de::DeserializeOwned, Serialize};

use super::asset_manager::AssetManager;

pub(crate) trait ReferenceSerializable<'rs> {
    type Serializable : Serialize + DeserializeOwned; 
    type Deserialized;
    
    fn serialize(self) -> Self::Serializable;

    fn deserialize(s: Self::Serializable, assets: &'rs AssetManager) -> Option<Self::Deserialized>;    
}