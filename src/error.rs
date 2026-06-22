use reqwest::Error;
use thiserror::Error;
use url::ParseError;

#[derive(Debug, Error)]
pub enum NyaaError {
    #[error("request error")]
    Request(#[from] Error),
    #[error("url error")]
    Url(#[from] ParseError),
}

pub type Result<T> = std::result::Result<T, NyaaError>;
