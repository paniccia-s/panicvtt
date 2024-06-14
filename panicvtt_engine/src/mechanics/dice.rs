use rand::RngCore;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Dice {
    D100    = 100, 
    D20     = 20, 
    D12     = 12, 
    D10     = 10,
    D8      = 8, 
    D6      = 6, 
    D4      = 4
}

#[cfg(test)]
pub type Rng = rand::rngs::mock::StepRng;

#[cfg(not(test))]
pub type Rng = rand::rngs::StdRng;

impl Dice { 
    pub fn roll(&self, rng: &mut Rng) -> u8 {
        let upper = *self as u8;
        (rng.next_u32() as u8 % upper) + 1
    }

    pub fn max(&self) -> u8 {
        *self as u8
    }
}


// pub fn roll_nonstandard(lower: u8, upper: u8) -> u8 {
//     rand::thread_rng().gen_range(lower..=upper)
// }

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    pub fn roll() {
        let dice = [Dice::D100, Dice::D20, Dice::D12, Dice::D10, Dice::D8, Dice::D6, Dice::D4];
        for die in dice {
            let upper = die as u8;
            let mut rng = StepRng::new(0, 5);

            for i in 0..1000u64 {
                // Expect to roll ((i * 5 % 255) % Dn) + 1 
                let roll = die.roll(&mut rng);
                let a = i * 5; 
                let b = a as u8; 
                let c = b % upper;
                let d = c.wrapping_add(1); 
                
                assert_eq!(roll, d);
            }
        }
    }

    #[test]
    pub fn max() {
        let expected = [100, 20, 12, 10, 8, 6, 4];
        let dice = [Dice::D100, Dice::D20, Dice::D12, Dice::D10, Dice::D8, Dice::D6, Dice::D4];

        for (e, d) in zip(expected, dice) {
            assert_eq!(d.max(), e);
        }
    }

    // #[test]
    // pub fn roll_nonstandard_bounds() {
    //     for lower in 0..u8::MAX - 1 {
    //         for upper in lower..u8::MAX {
    //             let roll = roll_nonstandard(lower, upper);
    //             assert!(roll >= lower && roll <= upper);
    //         }
    //     }
    // }
}
