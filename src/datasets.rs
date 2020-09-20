use crate::client::{ApifyClient, ApifyClientError, ApifyClientResult, IdOrName};
use crate::utils::{create_resource_locator, ResourceType};
use crate::generic_types::{SimpleBuilder, PaginationList, NoContent};
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::header::{HeaderMap, CONTENT_TYPE};

impl ApifyClient {
    /// List datasets of the provided account
    /// Requires API token
    pub fn list_datasets(&self) -> ListDatasetsBuilder<'_> {
        if self.optional_token.is_none() {
            panic!("list_datasets requires a token!");
        }
        ListDatasetsBuilder {
            client: self,
            options: ListDatasetsParams {
                offset: None,
                limit: None,
                desc: None,
                unnamed: None,
            }
        }
    }

    /// Requires API token
    pub fn create_dataset(&self, dataset_name: &str) -> SimpleBuilder<'_, Dataset> {
        if self.optional_token.is_none() {
            panic!("create_dataset requires a token!");
        }
        let url = format!("{}/datasets?name={}&token={}", self.base_path, dataset_name, self.optional_token.as_ref().unwrap());
        SimpleBuilder {
            client: self,
            url,
            method: reqwest::Method::POST,
            body: None,
            headers: None,
            phantom: PhantomData,
        }
    }

    /// Gets a dataset info object
    /// If you provide dataset ID, you don't need a token
    /// If you provide username~datasetName, you need a token (otherwise it will panic)
    pub fn get_dataset(&self, dataset_id_or_name: &IdOrName) -> SimpleBuilder<'_, Dataset> {
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}", self.base_path, dataset_id_or_name_val);
        let url_with_query = match &self.optional_token {
            None => url,
            Some(token) => format!("{}?token={}", &url, token)
        };
        println!("Constructed URL: {}", url_with_query);
        SimpleBuilder {
            client: self,
            url: url_with_query,
            method: reqwest::Method::GET,
            body: None,
            headers: None,
            phantom: PhantomData,
        }
    }

    /// Requires API token
    pub fn update_dataset(&self, dataset_id_or_name: &IdOrName) -> SimpleBuilder<'_, Dataset> {
        unimplemented!()
    }

    /// Requires API token
    pub fn delete_dataset(&self, dataset_id_or_name: &IdOrName) -> SimpleBuilder<'_, NoContent> {
        if self.optional_token.is_none() {
            panic!("delete_dataset requires a token!");
        }
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}?token={}", self.base_path, dataset_id_or_name_val, self.optional_token.as_ref().unwrap());
        SimpleBuilder {
            client: self,
            url,
            method: reqwest::Method::DELETE,
            body: None,
            headers: None,
            phantom: PhantomData,
        }
    }

    /// Requires API token
    pub fn put_items<T: Serialize>(&self, dataset_id_or_name: &IdOrName, items: &[T]) -> SimpleBuilder<'_, NoContent> {
        if self.optional_token.is_none() {
            panic!("put_items requires a token!");
        }
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}/items?token={}", self.base_path, dataset_id_or_name_val, self.optional_token.as_ref().unwrap());
        let bytes = serde_json::to_vec(items).unwrap();
        println!("bytes length: {}", bytes.len());
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        SimpleBuilder {
            client: self,
            url,
            method: reqwest::Method::POST,
            body: Some(bytes),
            headers: Some(headers),
            phantom: PhantomData,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    pub id: String,
    pub name: Option<String>,
    pub user_id: String,
    pub created_at: String,
    pub modified_at: String,
    pub accessed_at: String,
    pub item_count: u32,
    pub clean_item_count: Option<u32>,
    pub act_id: Option<String>,
    pub act_run_id: Option<String>
}

#[derive(QueryParams)]
struct ListDatasetsParams {
    offset: Option<u32>,
    limit: Option<u32>,
    desc: Option<bool>,
    unnamed: Option<bool>,
}

pub struct ListDatasetsBuilder<'a> {
    pub client: &'a ApifyClient,
    options: ListDatasetsParams
}

impl <'a> ListDatasetsBuilder<'a> {
    pub fn offset(& mut self, offset: u32) -> &'_ mut Self {
        self.options.offset = Some(offset);
        self
    }
    pub fn limit(& mut self, limit: u32) -> &'_ mut Self {
        self.options.limit = Some(limit);
        self
    }
    pub fn desc(& mut self, desc: bool) -> &'_ mut Self {
        self.options.desc = Some(desc);
        self
    }
    pub fn unnamed(& mut self, unnamed: bool) -> &'_ mut Self {
        self.options.unnamed = Some(unnamed);
        self
    }

    pub async fn send(&self) -> Result<PaginationList<Dataset>, ApifyClientError> {
        let mut query_string = self.options.to_query_params();
        if query_string.is_empty() {
            query_string = "?".to_string();
        }
        let url = format!("{}/datasets{}&token={}", self.client.base_path, query_string, self.client.optional_token.as_ref().unwrap());
        let resp = self.client.retrying_request(&url, &reqwest::Method::GET, &None, &None).await;
        match resp {
            Err(err) => Err(err),
            Ok(raw_data) => { 
                let apify_client_result: ApifyClientResult<PaginationList<Dataset>> = raw_data.json().await.unwrap();
                return Ok(apify_client_result.data);
            }
        }
    }
}