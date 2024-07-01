use std::{collections::HashMap, error::Error, fmt::{Debug, Display}};

use crate::assets::asset::Asset;

#[derive(Debug)]
pub struct AssetKeyError<'a, T> where T : Asset {
    message: String, 
    old_val: Box<T>, 
    new_val: &'a T,
    state: &'a HashMap<u128, T>,
}

impl<'a, T> AssetKeyError<'a, T> where T : Asset {
    pub fn new(message: String, old_val: T, new_val: &'a T, state: &'a HashMap<u128, T>) -> Self {
        Self {
            message, 
            old_val: Box::new(old_val), 
            new_val, state
        }
    }
}

impl<'a, T> Display for AssetKeyError<'a, T> where T : Asset + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.message)?;
        writeln!(f, "\t(Value to add: {}; present value: {}; UUID {})", self.old_val, self.new_val, self.old_val.get_uuid())?;
        writeln!(f, "\tProblematic HashMap: ")?;

        for (k, v) in self.state {
            writeln!(f, "\t\t{{ {}, {} }}", k, v)?;
        }

        Ok(())
    }
}

impl<'a, T> Error for AssetKeyError<'a, T> where T : Asset + Debug + Display  {
    
}