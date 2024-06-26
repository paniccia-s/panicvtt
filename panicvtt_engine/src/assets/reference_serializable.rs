
use serde::{de::DeserializeOwned, Serialize};

use super::asset_manager::AssetManager;

pub(crate) trait ReferenceSerializable<'rs> {
    // The intermediate type that is able to be directly serde'd with no intervention 
    type DirectSerde : Serialize + DeserializeOwned; 
    // The outer type that contains asset references that must be resolved before serde
    type TypeWithRefs;
    
    fn serialize(self) -> Self::DirectSerde;

    fn deserialize(s: Self::DirectSerde, assets: &'rs AssetManager) -> Option<Self::TypeWithRefs>;    
}