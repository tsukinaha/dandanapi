use std::borrow::Cow;

use crate::Request;

use super::{
    CommentResponse,
    MatchRequest,
    MatchResult,
    RequestComments,
    RequestEpisodes,
    SearchEpisodesResponse,
};

pub struct Match(pub MatchRequest);

impl Request for Match {
    type Response = MatchResult;
    type Body = MatchRequest;
    type Params = ();
    const PATH: &'static str = "/api/v2/match";
    const METHOD: reqwest::Method = reqwest::Method::POST;

    fn body(&self) -> Option<&Self::Body> {
        Some(&self.0)
    }
}

pub struct Episodes(pub RequestEpisodes);

impl Request for Episodes {
    type Response = SearchEpisodesResponse;
    type Body = ();
    type Params = RequestEpisodes;
    const PATH: &'static str = "/api/v2/search/episodes";

    fn params(&self) -> Option<&Self::Params> {
        Some(&self.0)
    }
}

pub struct Comments {
    pub episode_id: i64,
    pub request_comments: RequestComments,
}

impl Request for Comments {
    type Response = CommentResponse;
    type Body = ();
    type Params = RequestComments;
    const PATH: &'static str = "/api/v2/comment";

    fn params(&self) -> Option<&Self::Params> {
        Some(&self.request_comments)
    }

    fn path(&self) -> Cow<'static, str> {
        Cow::Owned(format!("{}/{}", Self::PATH, self.episode_id))
    }
}
