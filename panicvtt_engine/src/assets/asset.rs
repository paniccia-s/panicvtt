use serde::de::DeserializeOwned;


pub(crate) trait Asset : DeserializeOwned {
    fn get_uuid(&self) -> u128;
}