pub(crate) trait Asset {
    fn get_uuid(&self) -> u128;
    
    /// Retrieve the UUID of the campaign to which this Asset belongs. 
    /// 
    /// Returns None if this Asset is global to the VTT system.  
    fn get_owning_campaign(&self) -> Option<u128>;
}