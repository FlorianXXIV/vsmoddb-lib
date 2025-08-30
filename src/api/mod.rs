use reqwest::Url;

pub mod v1;

struct ModdbUrlBuilder {
    base: String,
}

impl ModdbUrlBuilder {
    pub fn new() -> ModdbUrlBuilder {
        let api_base_url = "http://mods.vintagestory.at/api";
        ModdbUrlBuilder {
            base: api_base_url.to_owned(),
        }
    }

    pub fn get(self) -> Url {
        Url::parse(&self.base).expect("parse")
    }

    pub fn tags(mut self) -> ModdbUrlBuilder {
        let tags = "/tags";
        self.base += tags;
        self
    }

    pub fn mods(mut self) -> ModdbUrlBuilder {
        let mods = "/mods";
        self.base += mods;
        self
    }

    pub fn authors(mut self) -> ModdbUrlBuilder {
        let authors = "/authors";
        self.base += authors;
        self
    }

    pub fn gameversions(mut self) -> ModdbUrlBuilder {
        let gameversions = "/gameversions";
        self.base += gameversions;
        self
    }

    pub fn r#mod(mut self, modid: u32) -> ModdbUrlBuilder {
        let api_mod = "/mod/";
        let id = modid.to_string();
        self.base = self.base + api_mod + &id;
        self
    }

    pub fn comments(mut self, assetid: Option<u32>) -> ModdbUrlBuilder {
        let comments = "/comments";
        let mut id = "".to_owned();
        if assetid.is_some() {
            id = "/".to_owned() + &assetid.unwrap().to_string();
        }
        self.base = self.base + comments + &id;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moddb_url_builder_base() {
        let builder = ModdbUrlBuilder::new().get();
        assert_eq!(builder.as_str(), "http://mods.vintagestory.at/api");
    }

    #[test]
    fn moddb_url_builder_tags() {
        let builder = ModdbUrlBuilder::new().tags().get();
        assert_eq!(builder.as_str(), "http://mods.vintagestory.at/api/tags")
    }

    #[test]
    fn moddb_url_builder_combine() {
        let builder = ModdbUrlBuilder::new().tags().mods().get();
        assert_eq!(
            builder.as_str(),
            "http://mods.vintagestory.at/api/tags/mods"
        )
    }
}
