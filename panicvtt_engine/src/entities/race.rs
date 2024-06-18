use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Race {
    name: String, 
    speed: u8, 
}

impl Race {
    pub fn new(name: String, speed: u8) -> Self {
        Self {
            name, speed
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name 
    }

    pub fn get_speed(&self) -> u8 {
        self.speed
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn getters() {
        let race = Race::new(String::from("Syd Barrett"), 123);

        assert_eq!(String::from("Syd Barrett"), race.get_name());
        assert_eq!(123, race.get_speed());
    }
}