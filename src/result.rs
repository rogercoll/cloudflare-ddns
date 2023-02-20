use cloudflare::framework::response::ApiFailure;
use std::fmt::Debug;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CloudflareError {
    #[error("invalid http client: `{0}`")]
    InvalidHttpClient(String),
    #[error("cloudflare API error: `{0}`")]
    ApiError(ApiFailure),
    #[error("more than one record was found")]
    MoreThanOneRecordFound,
}

#[derive(Error, Debug)]
pub enum FetchPublicIPError {
    #[error("request error: `{0}`")]
    RequestError(reqwest::Error),
    #[error("deserialize error: `{0}`")]
    DeserializeError(serde_json::Error),
    #[error("unable to find IP key in json")]
    NoIPKey,
    #[error("unable to parse IP address: `{0}`")]
    InvalidIPAddress(String),
}

impl From<reqwest::Error> for FetchPublicIPError {
    fn from(value: reqwest::Error) -> Self {
        FetchPublicIPError::RequestError(value)
    }
}

impl From<serde_json::Error> for FetchPublicIPError {
    fn from(value: serde_json::Error) -> Self {
        FetchPublicIPError::DeserializeError(value)
    }
}
