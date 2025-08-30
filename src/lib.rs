pub mod api;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::api::v1::{get_authors, get_gameversions, get_tags}; 

    use super::*;

    #[test]
    fn test_get_gameversions() {
        let versions = get_gameversions();
        
        assert!(versions.iter().any(|g| {g.get_name() == "1.21.0"}));
    }

    #[test]
    fn test_tags() {
        let tags = get_tags();
        assert!(tags.iter().any(|t| {t.get_name() == "Worldgen"}));
    }

    #[test]
    fn test_authors() {
        let authors = get_authors();

        assert_eq!(authors[0].get_name(), "Tyron");
    }
}
