### DanDanApi for Rust
This lib currently only serves the Tsukimi application.
So its completeness is very low.

### API Token Security
You shold use the `SecretGenerator` to generate a token for your application.

We use the `ed25519/X25519` algorithm to sign the token.

### Example
```rust
use dandanapi::{
    Comments,
    Episodes,
    SecretGenerator,
};

const X_APPID: &str = "e9imrhcexn";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = include_str!("../ed25519_key");
    let ciphertext = include_bytes!("../secret");
    let generator = SecretGenerator::new(ciphertext.to_vec(), key.to_string());
    dandanapi::DanDanClient::init(X_APPID.to_string(), generator).unwrap();
    let client = dandanapi::DanDanClient::instance();

    let route = client.route(Episodes(dandanapi::RequestEpisodes {
        anime: "夏日口袋".to_string(),
        tmdb_id: Some(271576),
        episode: "度过暑假的方法".to_string(),
    }));

    let response = route.await.unwrap();

    let episode_id = response
        .animes
        .unwrap()
        .first()
        .unwrap()
        .episodes
        .first()
        .unwrap()
        .episode_id;

    let route = client.route(Comments {
        episode_id,
        request_comments: dandanapi::RequestComments {
            from: 0,
            with_related: true,
            ch_convert: dandanapi::ChConvert::NONE,
        },
    });

    let response = route.await.unwrap();

    dbg!(response);

    Ok(())
}
```

### Contributing
The API structure is very scalable.

So add requests like this:
```rust
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
```


### License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.