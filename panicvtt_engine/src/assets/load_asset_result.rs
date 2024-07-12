use crate::util::asset_key_error::AssetKeyError;

use super::asset::Asset;


pub enum LoadAssetResult<'a, A> where A : Asset {
    Ok { asset: &'a A }, 
    UuidNotFoundError, 
    IoError { e: std::io::Error }, 
    NoCampaignFound,
    UuidDuplicateError { e: AssetKeyError<'a, A> }
}
