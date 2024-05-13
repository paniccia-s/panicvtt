use engine::Engine;

mod version;
pub mod entities;
pub mod engine;

pub fn version() -> &'static str {
    version::VERSION
}

pub fn initialize() -> Engine {
    Engine::new()
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
