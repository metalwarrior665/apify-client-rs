use crate::client::{ ApifyClient, ApifyClientResult, ApifyClientError };
use std::marker::PhantomData;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct PaginationList<T> {
    total: u32,
    offset: u32,
    limit: Option<u32>,
    count: u32,
    desc: bool,
    items: Vec<T>
}

pub struct SimpleBuilder <'a, T> {
    pub client: &'a ApifyClient,
    pub url: String,
    pub method: reqwest::Method,
    pub phantom: PhantomData<T>,
}

// Atempt at generic builder
impl<'a, T: serde::de::DeserializeOwned> SimpleBuilder<'a, T> {
    pub async fn send(&self) -> Result<T, ApifyClientError> {
        let resp = self.client.retrying_request(&self.url, &self.method, None, None).await;
        match resp {
            Err(err) => Err(err),
            Ok(raw_data) => { 
                let apify_client_result: ApifyClientResult<T> = raw_data.json().await.unwrap();
                return Ok(apify_client_result.data);
            }
        }
    }
}