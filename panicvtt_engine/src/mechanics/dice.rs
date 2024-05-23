use rand::Rng;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Dice {
    D100    = 100, 
    D20     = 20, 
    D12     = 12, 
    D10     = 10,
    D8      = 8, 
    D6      = 6, 
    D4      = 4
}

impl Dice {
    pub fn roll(&self) -> u8 {
        let upper = *self as u8;
        rand::thread_rng().gen_range(1..=upper)
    }
}


pub fn roll_nonstandard(lower: u8, upper: u8) -> u8 {
    rand::thread_rng().gen_range(lower..=upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn roll_bounds() {
        let dice = [Dice::D100, Dice::D20, Dice::D12, Dice::D10, Dice::D8, Dice::D6, Dice::D4];
        for die in dice {
            let upper = die as u8;
            for _ in 0..100000 {
                // Roll and check bounds 
                let roll = die.roll();
                assert!(roll > 0 && roll <= upper);
            }
        }
    }

    #[test]
    pub fn roll_nonstandard_bounds() {
        for lower in 0..u8::MAX - 1 {
            for upper in lower..u8::MAX {
                let roll = roll_nonstandard(lower, upper);
                assert!(roll >= lower && roll <= upper);
            }
        }
    }
}
