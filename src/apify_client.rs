use reqwest;
use serde::Deserialize;

use crate::resource_clients::run::{RunClient};

pub struct ApifyClient {
    // The token is optional
    pub optional_token: Option<String>,
    pub http_client: reqwest::Client,
    pub base_time_to_retry: u32,
    pub debug_log: bool,
    pub base_url: String,
}

#[derive(Deserialize, Debug)]
pub struct ApifyClientOutput<T> {
    pub data: T
}

impl ApifyClient {
    /// Creates a new Apify client with an optional token
    /// Be aware that all write operations requires token 
    /// Some read operations require token, some have optional token and some don't
    /// Using a method that requires token without a token in a client will result in Error
    pub fn new (optional_token: Option<String>) -> ApifyClient {
        let http_client = reqwest::Client::new();
        ApifyClient {
            optional_token,
            http_client,
            base_time_to_retry: 500,
            debug_log: true,
            base_url: "https://api.apify.com/v2".to_string(),
        }
    }

    pub fn run (&self, id_or_name: &str) -> RunClient {
        RunClient::new(self, id_or_name)
    }

    /// Sets a token on the client
    pub fn token (&mut self, token: String) -> () {
        self.optional_token = Some(token);
    }
}