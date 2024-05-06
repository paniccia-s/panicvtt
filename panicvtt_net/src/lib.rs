pub mod panicnet;
mod version; 

pub fn version() -> &'static str {
    version::VERSION
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version() {
        let version = super::version();
        assert_eq!(version, version::VERSION);
    }

}
