use reqwest;
use serde::Deserialize;

use std::fmt::{Display, Formatter};

pub const BASE_PATH: &str = "https://api.apify.com/v2";

pub struct ResourceName {
    pub user_name_or_user_id: String,
    pub resource_name: String
}

pub enum IdOrName {
    Id(String),
    Name(ResourceName),
}

pub struct ApifyClient {
    // The token is optional
    pub optional_token: Option<String>,
    pub http_client: reqwest::Client,
    pub base_time_to_retry: u32,
    pub debug_log: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApifyClientResult<T> {
    pub data: T
}

/// Errors returned by Apify API
#[derive(Debug, PartialEq)]
pub enum ApifyApiError {
    // The variant here is "type" and the param is "message"
    NotFound(String),
    // We don't have types for all statuses now so we just pass a message
    // TODO: Get rid of this after implementing all possibillitites
    RawError(String),
    MaxTimeoutRetriesReached(u8),
    MaxRateLimitRetriesReached(u8),
    MaxServerFailedRetriesReached(u8),
    /// Something is broken in the API or breaking change happened
    ApiFailure(String),
    MissingToken,
}

impl Display for ApifyApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Apify API returned an error: {:?}", self)
    }
}

impl std::error::Error for ApifyApiError {}

/// Errors can either be returned by Apify API or happen due to wrong JSON parsing logic.
/// Only few endpoints that include parsing JSON can return Parse error
#[derive(Debug)]
pub enum ApifyClientError {
    ApifyApi(ApifyApiError),
    Parse(serde_json::error::Error)
}

impl Display for ApifyClientError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApifyClientError::ApifyApi(apify_api_error) => write!(f, "{}", apify_api_error),
            ApifyClientError::Parse(parse_error) => write!(f, "JSON parsing failed, please fix your (de)serialization {}", parse_error),
        }
    }
}

impl std::error::Error for ApifyClientError {}

impl From<serde_json::error::Error> for ApifyClientError {
    fn from(e: serde_json::error::Error) -> Self {
        ApifyClientError::Parse(e)
    }
}

impl From<ApifyApiError> for ApifyClientError {
    fn from(e: ApifyApiError) -> Self {
        ApifyClientError::ApifyApi(e)
    }
}

impl ApifyClient {
    /// Creates a new Apify client with an optional token
    /// Be aware that all write operations requires token 
    /// Some read operations require token, some have optional token and some don't
    /// Using a method that requires token without a token in a client will result in Error
    pub fn new (optional_token: Option<String>) -> ApifyClient {
        if let Some(token) = &optional_token {
            assert_eq!(token.len(), 25);
        }
        let http_client = reqwest::Client::new();
        ApifyClient {
            optional_token,
            http_client,
            base_time_to_retry: 500,
            debug_log: true,
        }
    }

    /// Sets a token on the client
    pub fn token (&mut self, token: String) -> () {
        self.optional_token = Some(token);
    }
}