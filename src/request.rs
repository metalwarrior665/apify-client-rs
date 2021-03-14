use crate::client::{ApifyClient, ApifyApiError};
use tokio::time::sleep;
use std::time::Duration;
use serde::{Deserialize};

// TODO: Make this configurable
const MAX_RATE_LIMIT_RETRIES: u8 = 8;
const MAX_SERVER_FAIL_RETRIES: u8 = 8;
const MAX_TIMEOUT_RETRIES: u8 = 5;

#[derive(Deserialize, Debug)]
pub struct ApifyApiErrorRaw {
    r#type: String,
    message: String,
}

// TODO: Remove this
#[derive(Deserialize, Debug)]
pub struct ApifyApiErrorRawWrapper {
    error: ApifyApiErrorRaw
}

impl ApifyClient {
    async fn simple_request (
        &self,
        url: &str,
        method: &reqwest::Method,
        optional_body: &Option<Vec<u8>>,
        headers: &Option<reqwest::header::HeaderMap>
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut req_builder = match *method {
            reqwest::Method::GET => self.http_client.get(url),
            reqwest::Method::POST => self.http_client.post(url),
            reqwest::Method::PUT => self.http_client.put(url),
            reqwest::Method::DELETE => self.http_client.delete(url),
            // This error is only for the developer
            _ => panic!("Request method not usable with Apify API!"),
        };

        // TODO: Figure out how to remove the clones here
        if let Some(body) = optional_body.clone() {
            println!("Body size is: {}", body.len());
            req_builder = req_builder.body(body);
        }
        if let Some(headers) = headers.clone() {
            req_builder = req_builder.headers(headers);
        }
        req_builder.send().await
    }

    pub async fn retrying_request (
        &self,
        url: &str,
        method: &reqwest::Method,
        body: &Option<Vec<u8>>,
        headers: &Option<reqwest::header::HeaderMap>
    ) -> Result<reqwest::Response, ApifyApiError> {
        if self.debug_log {
            println!("Doing {} request to: {}", method, url);
        }
        let mut rate_limit_retry_count: u8 = 0;
        let mut server_failed_retry_count: u8 = 0;
        let mut timeout_retry_count: u8 = 0;
        loop {
            if rate_limit_retry_count >= MAX_RATE_LIMIT_RETRIES {
                return Err(ApifyApiError::MaxRateLimitRetriesReached(rate_limit_retry_count));
            }
            if server_failed_retry_count >= MAX_SERVER_FAIL_RETRIES {
                return Err(ApifyApiError::MaxServerFailedRetriesReached(server_failed_retry_count));
            }
            if timeout_retry_count >= MAX_TIMEOUT_RETRIES {
                return Err(ApifyApiError::MaxTimeoutRetriesReached(timeout_retry_count));
            }
            // TODO: Remove clones (moved in the loop), request could move back the body if should be retried
            match self.simple_request(url, method, body, headers).await {
                Ok(resp) => {
                    let status_code = resp.status().as_u16();
                    if status_code == 429 || status_code >= 500 {
                        let time_to_next_retry;
                        if status_code == 429 {
                            rate_limit_retry_count += 1;
                            // TODO: export this as separate func
                            time_to_next_retry = self.base_time_to_retry * (2 as u32).pow((rate_limit_retry_count).into());
                            if self.debug_log {
                                println!("Request got rate limit(429), retry n. will happen {} in: {} ms", rate_limit_retry_count, time_to_next_retry);
                            }
                        } else {
                            server_failed_retry_count += 1;
                            time_to_next_retry = self.base_time_to_retry * (2 as u32).pow((server_failed_retry_count).into());
                            if self.debug_log {
                                println!("Server failed({}), retry n. will happen {} in: {} ms", status_code, rate_limit_retry_count, time_to_next_retry);
                            }
                        }
                        
                        sleep(Duration::from_millis(time_to_next_retry.into())).await;
                        continue;
                    } else if status_code >= 300 {
                        let raw_error: ApifyApiErrorRawWrapper = resp.json().await.map_err(
                            |err| ApifyApiError::ApiFailure(format!("Apify API did not return correct error format. Something is very wrong. Please contact support@apify.com\n{}", err))
                        )?;
                        // error route
                        if status_code == 404 {
                            return Err(ApifyApiError::NotFound(raw_error.error.message));
                        }
                        return Err(ApifyApiError::RawError(raw_error.error.message));
                        // more types here
                    } else {
                        // ok route
                        return Ok(resp);
                    }
                }
                Err(err) => {
                    if err.is_timeout() {
                        timeout_retry_count += 1;
                        let time_to_next_retry = self.base_time_to_retry * (2 as u32).pow((timeout_retry_count).into());
                        if self.debug_log {
                            println!("Request timeouted, retry n. will happen {} in: {} ms", rate_limit_retry_count, time_to_next_retry);
                        }
                        sleep(Duration::from_millis(time_to_next_retry.into())).await;
                        continue;
                    }
                    // Maybe other types here
                    return Err(ApifyApiError::ApiFailure(format!("Uknown error, please create an issue on GitHub! {}", err)));
                }
            }
        }
    }
}