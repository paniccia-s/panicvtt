mod version;
pub mod entities;

pub fn version() -> &'static str {
    version::VERSION
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let version = version();
        assert_eq!(version, version::VERSION);
    }
}
