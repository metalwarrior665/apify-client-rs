use crate::client::{ApifyClient, ApifyApiError, ApifyClientError, ApifyClientResult};
use std::marker::PhantomData;
use serde::{Deserialize};

#[derive(Debug, PartialEq)]
pub struct NoContent;

impl NoContent {
    pub fn new() -> Self {
        NoContent {}
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct PaginationList<T> {
    pub total: u64,
    pub offset: u64,
    pub limit: Option<u64>,
    pub count: u64,
    pub desc: bool,
    pub items: Vec<T>
}

pub struct SimpleBuilder <'a, T> {
    pub client: &'a ApifyClient,
    pub url: String,
    pub requires_token: bool,
    pub method: reqwest::Method,
    // This is a bit weird, the parsing happens at the caller site and only Result is passed into send
    pub body: Result<Option<Vec<u8>>, serde_json::error::Error>,
    pub headers: Option<reqwest::header::HeaderMap>,
    pub phantom: PhantomData<T>,
}

// TODO: Ugly and hacky generic builder, try to figure out something better
impl<'a, T: serde::de::DeserializeOwned> SimpleBuilder<'a, T> {
    pub async fn send(self) -> Result<T, ApifyClientError> {
        let url = if self.requires_token {
            let token = self.client.optional_token.as_ref().ok_or(ApifyApiError::MissingToken)?;
            format!("{}&token={}", &self.url, token)
        } else {
            self.url
        };
        println!("size of: {}", std::mem::size_of::<T>());
        let body = self.body?;
        let resp = self.client.retrying_request(&url, &self.method, &body, &self.headers).await?;
        let bytes = resp.bytes().await.map_err(
            |err| ApifyApiError::ApiFailure(format!("Apify API did not return bytes. Something is very wrong. Please contact support@apify.com\n{}", err))
        )?;
        let apify_client_result: ApifyClientResult<T> = serde_json::from_slice(&bytes)?;
        Ok(apify_client_result.data)
            
    }
}

// TODO: Figure out if it is possible to remove this duplicated impl
impl<'a> SimpleBuilder<'a, NoContent> {
    pub async fn send(self) -> Result<NoContent, ApifyClientError> {
        let url = if self.requires_token {
            let token = self.client.optional_token.as_ref().ok_or(ApifyApiError::MissingToken)?;
            format!("{}&token={}", &self.url, token)
        } else {
            self.url
        };
        let body = self.body?;
        self.client.retrying_request(&url, &self.method, &body, &self.headers).await?;
        Ok(NoContent::new()) 
    }
}