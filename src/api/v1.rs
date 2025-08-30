use reqwest::{Url, blocking::get};
use serde::{Deserialize, Serialize};

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
    userid: u32,
    name: String,
}

impl Author {
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct VSMod {
    pub modid: u32,
    pub assetid: u32,
    pub name: String,
    pub text: String,
    pub author: String,
    pub urlalias: Option<String>,
    pub logofilename: Option<String>,
    pub logofile: Option<String>,
    pub logofiledb: Option<String>,
    pub homepageurl: Option<Url>,
    pub sourcecodeurl: Option<Url>,
    pub trailervideourl: Option<Url>,
    pub issuetrackerurl: Option<Url>,
    pub wikiurl: Option<Url>,
    pub downloads: u32,
    pub follows: u32,
    pub trendingpoints: u32,
    pub comments: u32,
    pub side: String,
    pub r#type: String,
    pub created: String,
    pub lastreleased: String,
    pub lastmodified: String,
    pub tags: Vec<String>,
    pub releases: Vec<Release>,
}

#[derive(Debug, Deserialize)]
pub struct Release {
    pub releaseid: u32,
    pub mainfile: Url,
    pub filename: String,
    pub fileid: u32,
    pub downloads: u32,
    pub tags: Vec<String>,
    pub modidstr: String,
    pub modversion: String,
    pub created: String,
    pub changelog: String,
}

#[derive(Debug, Deserialize)]
pub struct VSModQuery {
    pub modid: u32,
    pub assetid: u32,
    pub downloads: u32,
    pub follows: u32,
    pub trendingpoints: u32,
    pub comments: u32,
    pub name: String,
    pub summary: String,
    pub modidstrs: Vec<String>,
    pub author: String,
    pub urlalias: Option<String>,
    pub side: String,
    pub r#type: String,
    pub logo: String,
    pub tags: Vec<String>,
    pub lastreleased: String,
}

#[derive(Debug, Deserialize)]
pub struct VSComment {
    pub commentid: u32,
    pub assetid: u32,
    pub userid: u32,
    pub text: String,
    pub created: String,
    pub lastmodified: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    authors: Option<T>,
    gameversions: Option<T>,
    mods: Option<T>,
    r#mod: Option<T>,
    statuscode: String,
    tags: Option<T>,
    comments: Option<T>,
}

pub struct Client {
    client: reqwest::blocking::Client,
}

impl Client {
    pub fn new() -> Client {
        Client {
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_gameversions(&self) -> Vec<GameVersion> {
        let response: ApiResponse<Vec<GameVersion>> = self
            .client
            .get(ModdbUrlBuilder::new().gameversions().get())
            .send()
            .expect("get_gameversions")
            .json()
            .expect("json");

        response.gameversions.unwrap()
    }

    pub fn get_tags(&self) -> Vec<Tag> {
        let response: ApiResponse<Vec<Tag>> = self
            .client
            .get(ModdbUrlBuilder::new().tags().get())
            .send()
            .expect("get")
            .json()
            .expect("json");

        response.tags.unwrap()
    }

    pub fn get_authors(&self) -> Vec<Author> {
        let response: ApiResponse<Vec<Author>> = self
            .client
            .get(ModdbUrlBuilder::new().authors().get())
            .send()
            .expect("get")
            .json()
            .expect("json");

        response.authors.unwrap()
    }

    pub fn get_mod(&self, modid: u32) -> VSMod {
        let response: ApiResponse<VSMod> = self
            .client
            .get(ModdbUrlBuilder::new().r#mod(modid).get())
            .send()
            .expect("get")
            .json()
            .expect("json");

        response.r#mod.unwrap()
    }

    pub fn query_mods<T: Serialize + ?Sized>(&self, query: Option<&T>) -> Vec<VSModQuery> {
        let mut request = self.client.get(ModdbUrlBuilder::new().mods().get());
        if query.is_some() {
            request = request.query(query.unwrap());
        }

        let response: ApiResponse<Vec<VSModQuery>> =
            request.send().expect("send").json().expect("json");

        response.mods.unwrap()
    }

    pub fn get_comments(&self, assetid: Option<u32>) -> Vec<VSComment> {
        let response: ApiResponse<Vec<VSComment>> = self
            .client
            .get(ModdbUrlBuilder::new().comments(assetid).get())
            .send()
            .expect("send")
            .json()
            .expect("json");

        response.comments.unwrap()
    }
}
