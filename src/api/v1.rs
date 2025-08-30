use reqwest::blocking::get;
use serde::Deserialize;

use super::ModdbUrlBuilder;

#[derive(Debug, Deserialize)]
pub struct Tag {
    tagid: i64,
    name: String,
    color: String,
}

pub type GameVersion = Tag;
impl Tag {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct Author {
    userid: u64,
    name: String,
}

impl Author {
    pub fn get_name(&self) -> String{
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    statuscode: String,
    gameversions: Option<T>,
    tags: Option<T>,
    r#mod: Option<T>,
    mods: Option<T>,
    authors: Option<T>
}


pub fn get_gameversions() -> Vec<GameVersion> {
    let response: ApiResponse<Vec<GameVersion>> = get(ModdbUrlBuilder::new().gameversions().get())
        .expect("get_gameversions")
        .json()
        .expect("json");

    response.gameversions.unwrap()
}

pub fn get_tags() -> Vec<Tag> {
    let response: ApiResponse<Vec<Tag>> = get(ModdbUrlBuilder::new().tags().get())
        .expect("get")
        .json()
        .expect("json");

    response.tags.unwrap()
}

pub fn get_authors() -> Vec<Author> {
    let response: ApiResponse<Vec<Author>> = get(ModdbUrlBuilder::new().authors().get()).expect("get").json().expect("json");

    response.authors.unwrap()
}
