use crate::apify_client::{ApifyClient,  ApifyClientOutput};
use crate::error::{ApifyApiError, ApifyClientError, ClientValidationError};
use std::marker::PhantomData;
use crate::utils::parse_pagination_header;
use reqwest::header::HeaderMap;
use reqwest::Response;
use serde::{Deserialize};
use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct NoOutput;

impl NoOutput {
    pub fn new() -> Self {
        NoOutput {}
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

pub struct BaseBuilder <'a, OutputType> {
    client: &'a ApifyClient,
    url_segment: String,
    method: reqwest::Method,
    body: Option<Vec<u8>>,
    query_string: Option<String>,
    phantom: PhantomData<OutputType>,
}

// Base internal send for both Deserializable and NoOutput
impl <'a, OutputType> BaseBuilder<'a, OutputType> {
    pub fn new(client: &'a ApifyClient, url_segment: String, method: reqwest::Method) -> Self {
        BaseBuilder {
            client,
            url_segment,
            method,
            body: None,
            query_string: None,
            phantom: PhantomData,
        }
    }

    pub fn raw_payload(& mut self, payload: Vec<u8>) -> &'_ mut Self {
        self.body = Some(payload);
        self
    }

    // TODO: This proc macro crate only converts to string so adding new params is ugly
    pub fn append_query_string(& mut self, append_query_params: String) -> &'_ mut Self {
        if let Some(ref mut existing_query_string) = self.query_string {
            existing_query_string.push('&');
            existing_query_string.push_str(&append_query_params);
            self.query_string = Some(existing_query_string.clone());
            self
        } else {
            self.query_string = Some(append_query_params);
            self
        }
    }

    pub async fn validate_and_send_request(self) -> Result<Response, ApifyClientError> {
        let mut url = format!("{}/{}", self.client.base_url, self.url_segment);
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
        let apify_client_result: ApifyClientOutput<T> = serde_json::from_slice(&bytes)?;
        Ok(apify_client_result.data) 
    }

    pub async fn parse_pagination_list(self, resp: Response) -> Result<PaginationList<T>, ApifyClientError> {
        // For this endpoint, we have to reconstruct PaginationList manually
        let headers = resp.headers().clone();
        let bytes = resp.bytes().await.map_err(
            |err| ApifyApiError::ApiFailure(format!("Apify API did not return bytes. Something is very wrong. Please contact support@apify.com\n{}", err))
        )?;
        let items: Vec<T> = serde_json::from_slice(&bytes)?;
        println!("{:?}", headers);
        
        let total: u64 = parse_pagination_header(&headers, "X-Apify-Pagination-Total")?;
        let limit: u64 = parse_pagination_header(&headers, "X-Apify-Pagination-Limit")?;
        let offset: u64 = parse_pagination_header(&headers, "X-Apify-Pagination-Offset")?;
        // Because x-apify-pagination-count returns invalid values when hidden/empty items are skipped
        let count: u64 = items.len() as u64;

        let pagination_list = PaginationList {
            total,
            limit: Some(limit),
            count,
            offset,
            desc: false,
            items,
        };
        return Ok(pagination_list); 
    }
}

impl<'a> BaseBuilder<'a, NoOutput> {
    pub async fn send(self) -> Result<NoOutput, ApifyClientError> {
        self.validate_and_send_request().await?;
        Ok(NoOutput::new()) 
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

impl IdOrName {
    pub fn new(id_or_name: &str) -> Result<IdOrName, ApifyClientError> {
        // TODO: Check all allowed chars
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