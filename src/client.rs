use reqwest;
use serde::Deserialize;

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
    pub client: reqwest::Client,
    pub base_path: String,
    pub base_time_to_retry: u32,
    pub debug_log: bool,
}

#[derive(Deserialize, Debug)]
pub struct ApifyClientResult<T> {
    pub data: T
}


#[derive(Debug, PartialEq)]
pub enum ApifyClientError {
    // The variant here is "type" and the param is "message"
    NotFound(String),
    // We don't have types for all statuses now so we just pass a message
    RawError(String),
    MaxTimeoutRetriesReached(u8),
    MaxRateLimitRetriesReached(u8),
    MaxServerFailedRetriesReached(u8)
}

impl ApifyClient {
    /// Creates a new Apify client with an optional token
    /// Be aware that all write operations requires token 
    /// Some read operations require token, some have optional token and some don't
    /// Using a method that requires token without a token in a client will result in panic
    pub fn new (optional_token: Option<String>) -> ApifyClient {
        if let Some(token) = &optional_token {
            assert_eq!(token.len(), 25);
        }
        let client = reqwest::Client::new();
        ApifyClient {
            optional_token,
            client,
            base_path: "https://api.apify.com/v2".to_owned(),
            base_time_to_retry: 500,
            debug_log: true,
        }
    }

    /// Sets a token on the client
    pub fn token (&mut self, token: String) -> () {
        self.optional_token = Some(token);
    }
}