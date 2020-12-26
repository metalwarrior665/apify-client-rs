use crate::client::{ApifyClient, ApifyClientError, ApifyClientResult, IdOrName, BASE_PATH};
use crate::utils::{create_resource_locator, ResourceType, json_content_headers};
use crate::generic_types::{SimpleBuilder, PaginationList, NoContent};
use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
// use serde_json::Value;

impl ApifyClient {
    /// List datasets of the provided account
    /// Requires API token
    pub fn list_datasets(&self) -> ListDatasetsBuilder<'_> {
        if self.optional_token.is_none() {
            panic!("list_datasets requires a token!");
        }
        ListDatasetsBuilder {
            client: self,
            options: ListDatasetsParams::default(),
        }
    }

    /// Requires API token
    pub fn create_dataset(&self, dataset_name: &str) -> SimpleBuilder<'_, Dataset> {
        if self.optional_token.is_none() {
            panic!("create_dataset requires a token!");
        }
        let url = format!("{}/datasets?name={}&token={}", BASE_PATH, dataset_name, self.optional_token.as_ref().unwrap());
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
        let url = format!("{}/datasets/{}", BASE_PATH, dataset_id_or_name_val);
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
    pub fn update_dataset(&self, dataset_id_or_name: &IdOrName, new_dataset_name: &str) -> SimpleBuilder<'_, Dataset> {
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}?token={}", BASE_PATH, dataset_id_or_name_val, self.optional_token.as_ref().unwrap());
        let json_body = json!({
            "name": new_dataset_name
        });
        let bytes = serde_json::to_vec(&json_body).unwrap();
        SimpleBuilder {
            client: self,
            url,
            method: reqwest::Method::PUT,
            body: Some(bytes),
            headers: Some(json_content_headers()),
            phantom: PhantomData,
        }
    }

    /// Requires API token
    pub fn delete_dataset(&self, dataset_id_or_name: &IdOrName) -> SimpleBuilder<'_, NoContent> {
        if self.optional_token.is_none() {
            panic!("delete_dataset requires a token!");
        }
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}?token={}", BASE_PATH, dataset_id_or_name_val, self.optional_token.as_ref().unwrap());
        SimpleBuilder {
            client: self,
            url,
            method: reqwest::Method::DELETE,
            body: None,
            headers: None,
            phantom: PhantomData,
        }
    }

    /// Appends item(s) at the end of the dataset.
    /// `items` must serialize into JSON object or array of objects and the JSON must have size less than 5 MB.
    /// Otherwise the Apify API returns an error.
    /// Requires API token.
    /// [API reference](https://docs.apify.com/api/v2#/reference/datasets/item-collection/put-items)
    pub fn put_items<T: Serialize>(&self, dataset_id_or_name: &IdOrName, items: &T) -> SimpleBuilder<'_, NoContent> {
        if self.optional_token.is_none() {
            panic!("put_items requires a token!");
        }
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}/items?token={}", BASE_PATH, dataset_id_or_name_val, self.optional_token.as_ref().unwrap());
        let bytes = serde_json::to_vec(items).unwrap();
        println!("bytes length: {}", bytes.len());
        
        SimpleBuilder {
            client: self,
            url,
            method: reqwest::Method::POST,
            body: Some(bytes),
            headers: Some(json_content_headers()),
            phantom: PhantomData,
        }
    }

    /// Gets items from the dataset in JSON format and parses them into `PaginationList<T>`.
    /// If you need non-parsed String and/or different formats choose `get_items_raw` instead.
    /// [API reference](https://docs.apify.com/api/v2#/reference/datasets/item-collection/get-items).
    pub fn get_items<'de, T: serde::de::DeserializeOwned>(&self, dataset_id_or_name: &IdOrName) -> GetItemsBuilder<'_, T> {
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        GetItemsBuilder {
            client: self,
            dataset_id_or_name_val,
            options: GetItemsParams::default(),
            _phantom: PhantomData,
        }
    }

    /// Gets items from the dataset in any format and return them as `String` (no PaginationList). 
    /// [API reference](https://docs.apify.com/api/v2#/reference/datasets/item-collection/get-items).
    pub fn get_items_raw(&self, dataset_id_or_name: &IdOrName) -> GetItemsBuilderRaw<'_> {
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        GetItemsBuilderRaw {
            client: self,
            dataset_id_or_name_val,
            options: GetItemsParams::default(),
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

#[derive(Debug)]
pub enum Format {
    Json,
    Jsonl,
    Xml,
    Html,
    Csv,
    Xlsx,
    Rss,
}

impl Default for Format {
    fn default() -> Self {
        Format::Json
    }
}

impl std::fmt::Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        let string_repr = match self {
            Format::Json => "json",
            Format::Jsonl => "jsonl",
            Format::Xml => "xml",
            Format::Html => "html",
            Format::Csv => "csv",
            Format::Xlsx => "xlsx",
            Format::Rss => "ss",
        };
        write!(f, "{}", string_repr)
    }
}

#[derive(Default, QueryParams)]
#[allow(non_snake_case)]
struct GetItemsParams {
    format: Format,
    clean: Option<bool>,
    offset: Option<u64>,
    limit: Option<u64>,
    // Just string so QueryParams work, we parse it ourselves
    fields: Option<String>,
    // Just string so QueryParams work, we parse it ourselves
    omit: Option<String>,
    unwind: Option<String>,
    desc: Option<bool>, 
    attachment: Option<bool>,
    delimiter: Option<String>,
    bom: Option<bool>,
    xmlRoot: Option<String>,
    xmlRow: Option<String>,
    skipHeaderRow: Option<bool>,
    skipHidden: Option<bool>, 
    skipEmpty: Option<bool>, 
    simplified: Option<bool>,
    skipFailedPages: Option<bool>,
}

pub struct GetItemsBuilder<'a, T> {
    client: &'a ApifyClient,
    dataset_id_or_name_val: String,
    options: GetItemsParams,
    _phantom: PhantomData<T>,
}

pub struct GetItemsBuilderRaw<'a> {
    client: &'a ApifyClient,
    dataset_id_or_name_val: String,
    options: GetItemsParams,
}

impl <'a, T: DeserializeOwned> GetItemsBuilder<'a, T> {
    pub fn clean(& mut self, clean: bool) -> &'_ mut Self {
        self.options.clean = Some(clean);
        self
    }
    pub fn offset(& mut self, offset: u64) -> &'_ mut Self {
        self.options.offset = Some(offset);
        self
    }
    pub fn limit(& mut self, limit: u64) -> &'_ mut Self {
        self.options.limit = Some(limit);
        self
    }
    pub fn fields(& mut self, fields: Vec<String>) -> &'_ mut Self {
        self.options.fields = Some(fields.join(","));
        self
    }
    pub fn omit(& mut self, omit: Vec<String>) -> &'_ mut Self {
        self.options.omit = Some(omit.join(","));
        self
    }
    pub fn unwind(& mut self, unwind: String) -> &'_ mut Self {
        self.options.unwind = Some(unwind);
        self
    }
    pub fn desc(& mut self, desc: bool) -> &'_ mut Self {
        self.options.desc = Some(desc);
        self
    }
    pub fn attachment(& mut self, attachment: bool) -> &'_ mut Self {
        self.options.attachment = Some(attachment);
        self
    }
    pub fn delimiter(& mut self, delimiter: String) -> &'_ mut Self {
        self.options.delimiter = Some(delimiter);
        self
    }
    pub fn bom(& mut self, bom: bool) -> &'_ mut Self {
        self.options.bom = Some(bom);
        self
    }
    pub fn xml_root(& mut self, xml_root: String) -> &'_ mut Self {
        self.options.xmlRoot = Some(xml_root);
        self
    }
    pub fn xml_row(& mut self, xml_row: String) -> &'_ mut Self {
        self.options.xmlRow = Some(xml_row);
        self
    }
    pub fn skip_header_row(& mut self, skip_header_row: bool) -> &'_ mut Self {
        self.options.skipHeaderRow = Some(skip_header_row);
        self
    }
    pub fn skip_hidden(& mut self, skip_hidden: bool) -> &'_ mut Self {
        self.options.skipHidden = Some(skip_hidden);
        self
    }
    pub fn skip_empty(& mut self, skip_empty: bool) -> &'_ mut Self {
        self.options.skipEmpty = Some(skip_empty);
        self
    }
    pub fn simplified(& mut self, simplified: bool) -> &'_ mut Self {
        self.options.simplified = Some(simplified);
        self
    }
    pub fn skip_failed_pages(& mut self, skip_failed_pages: bool) -> &'_ mut Self {
        self.options.skipFailedPages = Some(skip_failed_pages);
        self
    }

    pub async fn send(&self) -> Result<PaginationList<T>, ApifyClientError> {
        let mut query_string = self.options.to_query_params();
        if query_string.is_empty() {
            query_string = "?".to_string();
        }
        let url = format!("{}/datasets/{}/items{}", BASE_PATH, self.dataset_id_or_name_val, query_string);
        let url_with_maybe_token = match &self.client.optional_token {
            None => url,
            Some(token) => format!("{}&token={}", &url, token)
        };
        let resp = self.client.retrying_request(&url_with_maybe_token, &reqwest::Method::GET, &None, &None).await;
        match resp {
            Err(err) => Err(err),
            Ok(raw_data) => { 
                // For this endpoint, we have to reconstruct PaginationList manually
                let headers = raw_data.headers().clone();
                let items: Vec<T> = raw_data.json().await.unwrap();

                println!("{:?}", headers);
                
                let total: u64 = headers.get("X-Apify-Pagination-Total").unwrap().to_str().unwrap().parse().unwrap();
                let limit: u64 = headers.get("X-Apify-Pagination-Limit").unwrap().to_str().unwrap().parse().unwrap();
                let offset: u64 = headers.get("X-Apify-Pagination-Offset").unwrap().to_str().unwrap().parse().unwrap();
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
    }
}

// TODO: Dedup this code
impl <'a> GetItemsBuilderRaw<'a> {
    pub fn format(& mut self, format: Format) -> &'_ mut Self {
        self.options.format = format;
        self
    }
    pub fn clean(& mut self, clean: bool) -> &'_ mut Self {
        self.options.clean = Some(clean);
        self
    }
    pub fn offset(& mut self, offset: u64) -> &'_ mut Self {
        self.options.offset = Some(offset);
        self
    }
    pub fn limit(& mut self, limit: u64) -> &'_ mut Self {
        self.options.limit = Some(limit);
        self
    }
    pub fn fields(& mut self, fields: Vec<String>) -> &'_ mut Self {
        self.options.fields = Some(fields.join(","));
        self
    }
    pub fn omit(& mut self, omit: Vec<String>) -> &'_ mut Self {
        self.options.omit = Some(omit.join(","));
        self
    }
    pub fn unwind(& mut self, unwind: String) -> &'_ mut Self {
        self.options.unwind = Some(unwind);
        self
    }
    pub fn desc(& mut self, desc: bool) -> &'_ mut Self {
        self.options.desc = Some(desc);
        self
    }
    pub fn attachment(& mut self, attachment: bool) -> &'_ mut Self {
        self.options.attachment = Some(attachment);
        self
    }
    pub fn delimiter(& mut self, delimiter: String) -> &'_ mut Self {
        self.options.delimiter = Some(delimiter);
        self
    }
    pub fn bom(& mut self, bom: bool) -> &'_ mut Self {
        self.options.bom = Some(bom);
        self
    }
    pub fn xml_root(& mut self, xml_root: String) -> &'_ mut Self {
        self.options.xmlRoot = Some(xml_root);
        self
    }
    pub fn xml_row(& mut self, xml_row: String) -> &'_ mut Self {
        self.options.xmlRow = Some(xml_row);
        self
    }
    pub fn skip_header_row(& mut self, skip_header_row: bool) -> &'_ mut Self {
        self.options.skipHeaderRow = Some(skip_header_row);
        self
    }
    pub fn skip_hidden(& mut self, skip_hidden: bool) -> &'_ mut Self {
        self.options.skipHidden = Some(skip_hidden);
        self
    }
    pub fn skip_empty(& mut self, skip_empty: bool) -> &'_ mut Self {
        self.options.skipEmpty = Some(skip_empty);
        self
    }
    pub fn simplified(& mut self, simplified: bool) -> &'_ mut Self {
        self.options.simplified = Some(simplified);
        self
    }
    pub fn skip_failed_pages(& mut self, skip_failed_pages: bool) -> &'_ mut Self {
        self.options.skipFailedPages = Some(skip_failed_pages);
        self
    }

    pub async fn send(&self) -> Result<String, ApifyClientError> {
        let mut query_string = self.options.to_query_params();
        if query_string.is_empty() {
            query_string = "?".to_string();
        }
        let url = format!("{}/datasets/{}/items{}", BASE_PATH, self.dataset_id_or_name_val, query_string);
        let url_with_maybe_token = match &self.client.optional_token {
            None => url,
            Some(token) => format!("{}&token={}", &url, token)
        };
        let resp = self.client.retrying_request(&url_with_maybe_token, &reqwest::Method::GET, &None, &None).await;
        match resp {
            Err(err) => Err(err),
            Ok(raw_data) => { 
                let output = raw_data.text().await.unwrap();
                return Ok(output);
            }
        }
    }
}

#[derive(QueryParams, Default)]
struct ListDatasetsParams {
    offset: Option<u32>,
    limit: Option<u32>,
    desc: Option<bool>,
    unnamed: Option<bool>,
}

pub struct ListDatasetsBuilder<'a> {
    client: &'a ApifyClient,
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
        let url = format!("{}/datasets{}&token={}", BASE_PATH, query_string, self.client.optional_token.as_ref().unwrap());
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