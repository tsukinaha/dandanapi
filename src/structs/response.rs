use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchResult {
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "isMatched")]
    pub is_matched: bool,
    #[serde(rename = "matches")]
    pub matches: Option<Vec<MatchEntry>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchEntry {
    #[serde(rename = "episodeId")]
    pub episode_id: i64,
    #[serde(rename = "animeId")]
    pub anime_id: i32,
    #[serde(rename = "animeTitle")]
    pub anime_title: Option<String>,
    #[serde(rename = "episodeTitle")]
    pub episode_title: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    #[serde(rename = "shift")]
    pub shift: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEpisodesResponse {
    #[serde(rename = "errorCode")]
    pub error_code: i32,
    #[serde(rename = "success")]
    pub success: bool,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>,
    #[serde(rename = "animes")]
    pub animes: Option<Vec<SearchEpisodesAnime>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEpisodesAnime {
    #[serde(rename = "animeId")]
    pub anime_id: i32,
    #[serde(rename = "animeTitle")]
    pub anime_title: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "typeDescription")]
    pub type_description: Option<String>,
    #[serde(rename = "episodes")]
    pub episodes: Vec<SearchEpisodeDetails>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchEpisodeDetails {
    #[serde(rename = "episodeId")]
    pub episode_id: i64,
    #[serde(rename = "episodeTitle")]
    pub episode_title: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentResponse {
    #[serde(rename = "count")]
    pub count: i32,
    #[serde(rename = "comments")]
    pub comments: Option<Vec<CommentData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommentData {
    #[serde(rename = "cid")]
    pub cid: i64,
    #[serde(rename = "p")]
    pub p: Option<String>,
    #[serde(rename = "m")]
    pub m: Option<String>,
}
