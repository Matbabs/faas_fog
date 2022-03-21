use std::fmt::Display;

use serde::{self, Deserialize};

#[derive(Debug, thiserror::Error)]
pub enum Error<T>
where
    T: Display,
{
    #[error("Reqwest failed with error: {0}")]
    Reqwest(reqwest::Error),
    #[error("Serde failed with error: {0}")]
    Serde(serde_json::Error),
    #[error("The API request responded an unexpected payload")]
    Api(ApiError<T>),
}

#[derive(Debug)]
pub struct ApiError<T>
where
    T: Display,
{
    pub code: reqwest::StatusCode,
    pub content: T,
}

impl<'de, T> From<(reqwest::StatusCode, Result<T, reqwest::Error>)> for Error<T>
where
    T: Deserialize<'de> + Display,
{
    fn from(err: (reqwest::StatusCode, Result<T, reqwest::Error>)) -> Self {
        match err.1 {
            Ok(content) => Error::Api(ApiError {
                code: err.0,
                content,
            }),
            Err(err) => Error::from(err),
        }
    }
}

impl<T> From<reqwest::Error> for Error<T>
where
    T: Display,
{
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl<T> From<serde_json::Error> for Error<T>
where
    T: Display,
{
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

pub mod models;

mod default_api;
pub use self::default_api::{DefaultApi, DefaultApiClient};

pub mod configuration;
pub use self::configuration::Configuration;
