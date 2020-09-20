use crate::client::{ ApifyClient, ApifyClientResult, ApifyClientError };
use std::marker::PhantomData;
use serde::{Deserialize};

#[derive(Debug, PartialEq)]
pub struct NoContent;

impl NoContent {
    pub fn new() -> Self {
        NoContent {}
    }
}

#[derive(Deserialize, Debug)]
pub struct PaginationList<T> {
    total: u32,
    offset: u32,
    limit: Option<u32>,
    count: u32,
    desc: bool,
    pub items: Vec<T>
}

pub struct SimpleBuilder <'a, T> {
    pub client: &'a ApifyClient,
    pub url: String,
    pub method: reqwest::Method,
    pub body: Option<Vec<u8>>,
    pub headers: Option<reqwest::header::HeaderMap>,
    pub phantom: PhantomData<T>,
}

// Atempt at generic builder
impl<'a, T: serde::de::DeserializeOwned> SimpleBuilder<'a, T> {
    pub async fn send(&self) -> Result<T, ApifyClientError> {
        println!("size of: {}", std::mem::size_of::<T>());
        let resp = self.client.retrying_request(&self.url, &self.method, &self.body, &self.headers).await;
        match resp {
            Err(err) => Err(err),
            Ok(resp) => { 
                let apify_client_result: ApifyClientResult<T> = resp.json().await.unwrap();
                return Ok(apify_client_result.data);
            }    
        }
    }
}

// TODO: Figure out if it is possible to remove this duplicated impl
impl<'a> SimpleBuilder<'a, NoContent> {
    pub async fn send(&self) -> Result<NoContent, ApifyClientError> {
        let resp = self.client.retrying_request(&self.url, &self.method, &self.body, &self.headers).await;
        match resp {
            Err(err) => Err(err),
            Ok(_) => { 
                Ok(NoContent::new())
            }    
        }
    }
}