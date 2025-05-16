use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MatchRequest {
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "fileHash")]
    pub file_hash: Option<String>,
    #[serde(rename = "fileSize")]
    pub file_size: i64,
    #[serde(rename = "videoDuration")]
    pub video_duration: i32,
    #[serde(rename = "matchMode")]
    pub match_mode: MatchMode,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MatchMode {
    #[serde(rename = "hashAndFileName")]
    HashAndFileName,
    #[serde(rename = "fileNameOnly")]
    FileNameOnly,
    #[serde(rename = "hashOnly")]
    HashOnly,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestEpisodes {
    #[serde(rename = "anime")]
    pub anime: String,
    #[serde(rename = "tmdbId")]
    pub tmdb_id: Option<i32>,
    #[serde(rename = "episode")]
    pub episode: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestComments {
    #[serde(rename = "from")]
    pub from: i64,
    #[serde(rename = "withRelated")]
    pub with_related: bool,
    #[serde(rename = "chConvert")]
    pub ch_convert: ChConvert,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChConvert {
    NONE = 0,
    SIMPLIFIED = 1,
    TRADITIONAL = 2,
}
