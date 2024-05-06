static VERSION: &str = "pre-natal";

pub fn version() -> &'static str {
    VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let version = version();
        assert_eq!(version, VERSION);
    }
}
