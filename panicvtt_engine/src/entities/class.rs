use crate::mechanics::dice::Dice;

#[derive(Clone)]
pub struct Class {
    name: String, 
    hit_die: Dice
}

impl Class {

    pub fn new(name: String, hit_die: Dice) -> Self {
        Self {
            name, hit_die
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_hit_die(&self) -> Dice {
        self.hit_die
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn getters() {
        let class = Class::new(String::from("Nick Mason"), Dice::D8);
        
        assert_eq!(String::from("Nick Mason"), class.get_name());
        assert_eq!(Dice::D8, class.get_hit_die());
    }
}