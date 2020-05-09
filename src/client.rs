use reqwest;
use tokio::time::delay_for;
use std::time::Duration;

const MAX_RATE_LIMIT_RETRIES: u8 = 8;
const MAX_SERVER_FAIL_RETRIES: u8 = 8;

pub struct ApifyClient {
    pub token: String,
    pub client: reqwest::Client,
    pub base_path: String
}

pub struct ApifyApiErrorRaw {
    r#type: String,
    message: String,
}

pub enum ApifyClientError {
    Timeout,
    // The variant here is "type" and the param is "message"
    NotFound(String),
    MaxRateLimitRetriesReached(u8),
    MaxServerFailedRetriesReached(u8)
}

impl ApifyClient {
    pub fn new (token: String) -> ApifyClient {
        assert_eq!(token.len(), 25);
        let client = reqwest::Client::new();
        ApifyClient {
            token,
            client,
            base_path: "https://api.apify.com/v2".to_owned(),
        }
    }

    async fn simple_request (&self, url: &str, method: reqwest::Method, body: Vec<u8>, headers: reqwest::header::HeaderMap) -> Result<reqwest::Response, reqwest::Error> {
        match method {
            reqwest::Method::GET => self.client.get(url).send().await,
            reqwest::Method::POST => self.client.get(url).body(body).headers(headers).send().await,
            reqwest::Method::PUT => self.client.get(url).body(body).headers(headers).send().await,
            reqwest::Method::DELETE => self.client.get(url).send().await,
        }
    }

    async fn retrying_request (&self, url: &str, method: reqwest::Method, body: Vec<u8>, headers: reqwest::header::HeaderMap) -> Result<Vec<u8>, ApifyClientError> {
        let mut time_to_next_retry_ms: u64 = 500;
        let mut rate_limit_retry_count: u8 = 0;
        let mut server_failed_retry_count: u8 = 0;
        loop {
            if rate_limit_retry_count >= MAX_RATE_LIMIT_RETRIES {
                return Err(ApifyClientError::MaxRateLimitRetriesReached(rate_limit_retry_count));
            }
            if server_failed_retry_count >= MAX_SERVER_FAIL_RETRIES {
                return Err(ApifyClientError::MaxServerFailedRetriesReached(server_failed_retry_count));
            }
            match self.simple_request(url, method, body, headers).await {
                Ok(resp) => {
                    let status_code = resp.status().as_u16();
                    if status_code == 429 || status_code >= 500 {
                        // TODO: Here we should fine tune this, split the backoff for rate limit and server errors
                        delay_for(Duration::from_millis(time_to_next_retry_ms)).await;
                        if status_code == 429 {
                            rate_limit_retry_count += 1;
                        } else {
                            server_failed_retry_count += 1;
                        }
                        time_to_next_retry_ms = time_to_next_retry_ms * (2 as u64).pow((rate_limit_retry_count + server_failed_retry_count).into()) ;
                        continue;
                    } else if status_code >= 300 {
                        let raw_error: ApifyApiErrorRaw = resp.json()
                        // error route
                        if status_code == 404 {
                            return Err(ApifyClientError::NotFound(message));
                        }
                        // more types here
                    } else {
                        // ok route
                        return Ok(resp.bytes().await?.to_vec());
                    }
                }
                Err(err) => {
                    if err.is_timeout() {
                        return Err(ApifyClientError::Timeout);
                    }
                    // Maybe other types here
                    panic!("ApifyClientError: Uknown error, please create an issue on GitHub! {}", err);
                }
            }
        }
    }
}