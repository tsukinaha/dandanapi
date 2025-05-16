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

    println!("Episode ID: {}", episode_id);

    let route = client.route(Comments {
        episode_id,
        request_comments: dandanapi::RequestComments {
            from: 0,
            with_related: true,
            ch_convert: dandanapi::ChConvert::NONE,
        },
    });

    let response = route.await.unwrap();

    println!("Response: {:?}", response);

    Ok(())
}
