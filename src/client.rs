use std::{
    borrow::Cow,
    sync::{
        LazyLock,
        OnceLock,
    },
};

use futures::{
    FutureExt,
    future::BoxFuture,
};
use reqwest::{
    Client,
    Method,
};

use crate::{
    Result,
    error::Error,
    secret::{
        RequestHeaderGenerator,
        SecretGenerator,
    },
};

static HEADER_GENERATOR: OnceLock<RequestHeaderGenerator> = OnceLock::new();
const BASE_URI: &str = "https://api.dandanplay.net";

#[derive(Clone, Debug, Default)]
pub struct DanDanClient(ClientInner);

impl DanDanClient {
    pub fn init(x_appid: String, secret_generator: SecretGenerator) -> Result<()> {
        let request_header_generator = RequestHeaderGenerator::new(x_appid, secret_generator)?;
        HEADER_GENERATOR
            .set(request_header_generator)
            .map_err(|_| Error::SecretGenerationError("Already initialized".into()))?;
        Ok(())
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self(ClientInner::with_client(client))
    }

    pub fn instance() -> Self {
        static INSTANCE: LazyLock<DanDanClient> = LazyLock::new(DanDanClient::default);
        INSTANCE.clone()
    }

    pub fn route<T>(&self, kind: T) -> Route<T> {
        Route::new(&self.0.client, kind)
    }
}

#[derive(Clone, Debug)]
struct ClientInner {
    client: reqwest::Client,
}

impl Default for ClientInner {
    fn default() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }
}

impl ClientInner {
    fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}
pub struct Route<T> {
    client: Client,
    kind: T,
}

impl<T> Route<T> {
    pub fn new(client: &Client, kind: T) -> Self {
        Self {
            client: client.clone(),
            kind,
        }
    }
}

impl<T: Request> std::fmt::Display for Route<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.write_str(T::PATH)
    }
}

impl<T: Request> IntoFuture for Route<T> {
    type Output = Result<T::Response>;
    type IntoFuture = BoxFuture<'static, Self::Output>;

    fn into_future(self) -> Self::IntoFuture {
        async move {
            let path = self.kind.path();

            let mut request = self
                .client
                .request(T::METHOD, format!("{BASE_URI}{path}"))
                .headers(
                    HEADER_GENERATOR
                        .get()
                        .ok_or(Error::SecretGenerationError(
                            "Header generator not initialized".into(),
                        ))?
                        .header(&path)?,
                );

            if let Some(body) = self.kind.body() {
                request = request.json(&body);
            }

            if let Some(params) = self.kind.params() {
                request = request.query(&params);
            }

            let response = request.send().await.map_err(Error::HttpError)?;

            let data = response
                .json::<T::Response>()
                .await
                .map_err(Error::HttpError)?;

            Ok(data)
        }
        .boxed()
    }
}

pub trait Request: Sized + Send + 'static {
    type Response: serde::de::DeserializeOwned + Send + 'static;
    type Body: serde::ser::Serialize;
    type Params: serde::ser::Serialize;

    const METHOD: Method = Method::GET;
    const PATH: &'static str;

    fn body(&self) -> Option<&Self::Body> {
        None
    }

    fn params(&self) -> Option<&Self::Params> {
        None
    }

    fn path(&self) -> Cow<'static, str> {
        Cow::Borrowed(Self::PATH)
    }
}
