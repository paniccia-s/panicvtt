use std::path::Path;

use engine::Engine;

pub mod assets;
pub mod campaigns;
pub mod entities;
pub mod engine;
mod mechanics;
mod util;
mod version;

pub fn version() -> &'static str {
    version::VERSION
}

#[cfg(test)]
pub fn initialize(asset_root: &Path) -> Engine {
    use rand::rngs::mock::StepRng;

    Engine::new(StepRng::new(0, 1), asset_root)
}

#[cfg(not(test))]
pub fn initialize(asset_root: &Path) -> Engine {
    use rand::{rngs::StdRng, SeedableRng};

    Engine::new(StdRng::from_entropy(), asset_root)
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
