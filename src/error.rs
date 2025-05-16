#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("http: {0}")]
    HttpError(reqwest::Error),
    #[error("serde: {0}")]
    SerdeError(serde_json::Error),
    #[error("secret: {0}")]
    SecretGenerationError(String),
}
