use engine::Engine;

mod version;
pub mod entities;
pub mod engine;
mod mechanics;
mod util;

pub fn version() -> &'static str {
    version::VERSION
}

#[cfg(test)]
pub fn initialize() -> Engine {
    use rand::rngs::mock::StepRng;

    Engine::new(StepRng::new(0, 1))
}

#[cfg(not(test))]
pub fn initialize() -> Engine {
    use rand::{rngs::StdRng, SeedableRng};

    Engine::new(StdRng::from_entropy())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert_eq!(version, version::VERSION);
    }
}
