pub mod api;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use crate::api::v1::Client;

    #[test]
    fn test_get_gameversions() {
        let versions = Client::new().get_gameversions();

        assert!(versions.iter().any(|g| { g.get_name() == "1.21.0" }));
    }

    #[test]
    fn test_tags() {
        let tags = Client::new().get_tags();
        assert!(tags.iter().any(|t| { t.get_name() == "Worldgen" }));
    }

    #[test]
    fn test_authors() {
        let authors = Client::new().get_authors();

        assert_eq!(authors[0].get_name(), "Tyron");
    }

    #[test]
    fn test_mod() {
        let vsmod = Client::new().get_mod(6);

        assert_eq!(vsmod.name, "More Variants");
    }

    #[test]
    fn test_query() {
        let hits = Client::new().query_mods(Some(&[
            ("text", "jack"),
            ("tagids[]", "7"),
            ("tagids[]", "8"),
            ("orderby", "downloadw"),
        ]));

        assert_eq!(hits[0].modid, 160);
    }

    #[test]
    fn test_comments() {
        let commetns = Client::new().get_comments(Some(1));

        assert_eq!(commetns[0].text, "<p>The first mod...</p>")
    }
}
