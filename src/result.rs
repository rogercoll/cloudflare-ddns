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
