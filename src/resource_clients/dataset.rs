use crate::apify_client::ApifyClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::base_clients::resource_client::ResourceClient;
use crate::generic_types::{BaseBuilder, PaginationList, NoOutput};
use crate::error::ApifyClientError;
use crate::builders::dataset::{GetItemsBuilder, DownloadItemsBuilder, Format};
use std::fmt::format;
use std::marker::PhantomData;

pub struct DatasetClient<'a> {
    pub apify_client: &'a ApifyClient,
    pub url_segment: String,
}

// See comment on the ResourceClient trait why this boilerplate is needed
impl <'a> ResourceClient<'a, Dataset> for DatasetClient<'a> {
    fn get_client(&self) -> &'a ApifyClient {
        self.apify_client
    }

    fn get_url_segment(&self) -> &str {
        &self.url_segment
    }
}

impl <'a> DatasetClient<'a> {
    pub fn new(apify_client: &'a ApifyClient, identifier: &str) -> Self {
        DatasetClient {
            apify_client,
            url_segment: format!("dataset/{}", identifier),
        }
    }

    pub fn list_items<T: serde::de::DeserializeOwned>(&self) -> GetItemsBuilder<T> {
        GetItemsBuilder::new(self)
    }

    pub fn download_items(&self, format: Format) -> DownloadItemsBuilder {
        DownloadItemsBuilder::new(self, format)
    }

    // TODO: Pass items by reference, figure out lifetimes
    pub fn push_items<T: serde::Serialize> (&self, items: T) -> PushItemsBuilder<T> {
        PushItemsBuilder{
            dataset_client: self,
            items: items,
        }
    }

    pub fn update(&self, name: &str) -> UpdateDatasetBuilder {
        UpdateDatasetBuilder {
            dataset_client: self,
            payload: UpdateDatasetPayload {
                name: name.to_owned(),
            },
        }
    }
}

pub struct PushItemsBuilder<'a, T: serde::Serialize + 'a> {
    dataset_client: &'a DatasetClient<'a>,
    items: T,
}

impl <'a, T: serde::Serialize> PushItemsBuilder<'a, T> {
    pub async fn send(self) -> Result<NoOutput, ApifyClientError> {
        let mut builder: BaseBuilder<'_, NoOutput> = BaseBuilder::new(
            self.dataset_client.apify_client,
            self.dataset_client.url_segment.clone(),
            Method::POST,
        );
        builder.raw_payload(serde_json::to_vec(&self.items)?);
        builder.validate_and_send_request().await?;
        Ok(NoOutput)
    }
}

#[derive(Serialize, Debug)]
pub struct UpdateDatasetPayload {
    name: String
}

pub struct UpdateDatasetBuilder<'a> {
    dataset_client: &'a DatasetClient<'a>,
    payload: UpdateDatasetPayload,
}

impl <'a> UpdateDatasetBuilder<'a> {
    pub async fn send(self) -> Result<Dataset, ApifyClientError> {
        let mut builder: BaseBuilder<'_, Dataset> = BaseBuilder::new(
            self.dataset_client.apify_client,
            self.dataset_client.url_segment.clone(),
            Method::PUT,
        );
        builder.raw_payload(serde_json::to_vec(&self.payload)?);
        builder.send().await
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
