use crate::apify_client::{ApifyClient,  ApifyClientOutput};
use crate::error::{ApifyApiError, ApifyClientError, ClientValidationError};
use std::marker::PhantomData;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::{Deserialize};
use regex::Regex;

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
        let apify_client_result: ApifyClientOutput<T> = serde_json::from_slice(&bytes)?;
        Ok(apify_client_result.data)
            
    }
}

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

pub struct BaseBuilder <'a, T> {
    client: &'a ApifyClient,
    url_segment: String,
    identifier: String,
    method: reqwest::Method,
    // This is a bit weird, the parsing happens at the caller site and only Result is passed into send
    body: Option<Vec<u8>>,
    query_string: Option<String>,
    phantom: PhantomData<T>,
}

// Base internal send for both Deserializable and NoContent
impl <'a, T> BaseBuilder<'a, T> {
    pub fn new(client: &'a ApifyClient, url_segment: String, identifier: String, method: reqwest::Method) -> Self {
        BaseBuilder {
            client,
            url_segment,
            identifier,
            method,
            body: None,
            query_string: None,
            phantom: PhantomData,
        }
    }

    // TODO: We proably don't want to parse during the builder so this is temporary
    pub fn json<J: serde::Serialize>(& mut self, payload_type: &J) -> Result<&'_ mut Self, serde_json::error::Error> {
        self.body = Some(serde_json::to_vec(payload_type)?);
        Ok(self)
    }

    pub fn raw_payload(& mut self, payload: Vec<u8>) -> &'_ mut Self {
        self.body = Some(payload);
        self
    }

    pub fn query_string(& mut self, query_string: String) -> &'_ mut Self {
        self.query_string = Some(query_string);
        self
    }

    async fn validate_and_send_request(self) -> Result<Response, ApifyClientError> {
        let id_or_name = IdOrName::new(&self.identifier)?;

        let requires_token = match self.method {
            reqwest::Method::GET => {
                match id_or_name {
                    IdOrName::Id(_) => false,
                    IdOrName::Name(_) => true,
                }
            },
            _ => true,
        };
        if requires_token && self.client.optional_token.is_none() {
            return Err(ClientValidationError::MissingToken.into());
        }
        let mut url = format!("{}/{}/{}", self.client.base_url, self.url_segment, id_or_name.to_string());
        if let Some(query_string) = self.query_string {
            url = format!("{}?{}", url, query_string);
        }
        // println!("size of: {}", std::mem::size_of::<T>());
        let body = self.body;
        let resp = self.client.retrying_request(&url, &self.method, &body, &Some(HeaderMap::new())).await?;
        Ok(resp)
    }
}

impl<'a, T: serde::de::DeserializeOwned> BaseBuilder<'a, T> {
    pub async fn send(self) -> Result<T, ApifyClientError> {
        let resp = self.validate_and_send_request().await?;
        let bytes = resp.bytes().await.map_err(
            |err| ApifyApiError::ApiFailure(format!("Apify API did not return bytes. Something is very wrong. Please contact support@apify.com\n{}", err))
        )?;
        // For debugging
        let string = std::str::from_utf8(&bytes).unwrap();
        println!("response body: {}", string);
        let apify_client_result: ApifyClientOutput<T> = serde_json::from_slice(&bytes)?;
        Ok(apify_client_result.data) 
    }
}

impl<'a> BaseBuilder<'a, NoContent> {
    pub async fn send(self) -> Result<NoContent, ApifyClientError> {
        self.validate_and_send_request().await?;
        Ok(NoContent::new()) 
    }
}

#[derive(Clone)]
pub struct ResourceName {
    pub user_name_or_user_id: String,
    pub resource_name: String
}

#[derive(Clone)]
pub enum IdOrName {
    Id(String),
    Name(ResourceName),
}

// TODO: check all allowed chars


impl IdOrName {
    pub fn new(id_or_name: &str) -> Result<IdOrName, ApifyClientError> {
        let resource_name_regex: Regex = Regex::new(r"[A-Za-z0-9-_.]/[A-Za-z0-9-_.]").unwrap();
        let resource_id_regex: Regex = Regex::new(r"[A-Za-z0-9]{17}").unwrap();
        if resource_id_regex.is_match(id_or_name) {
            Ok(IdOrName::Id(id_or_name.to_string()))
        } else if resource_name_regex.is_match(id_or_name) {
            let mut split = id_or_name.split('/');
            let user_name_or_user_id = split.next().unwrap().to_string();
            let resource_name = split.next().unwrap().to_string();
            Ok(IdOrName::Name(ResourceName { user_name_or_user_id, resource_name }))
        } else {
            Err(ClientValidationError::InvalidResourceIdOrName(
                format!("Resource needs to be either an ID with 17 chars or a name with a slash. Got: {}", id_or_name)
            ).into())
        }
    }
        

    pub fn to_string(self) -> String {
        match self {
            IdOrName::Id(id) => {
                id
            },
            IdOrName::Name(resource_name) => {
                format!("{}/{}", resource_name.user_name_or_user_id, resource_name.resource_name)
            },
        }
    }
}