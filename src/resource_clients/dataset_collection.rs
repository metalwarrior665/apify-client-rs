use crate::apify_client::ApifyClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use crate::base_clients::resource_client::ResourceClient;
use crate::generic_types::{BaseBuilder, PaginationList, NoOutput};
use crate::error::ApifyClientError;
use crate::builders::dataset::{GetItemsBuilder, DownloadItemsBuilder, Format};
use std::marker::PhantomData;

pub struct DatasetCollectionClient<'a> {
    pub apify_client: &'a ApifyClient,
    pub url_segment: String,
}

// See comment on the ResourceClient trait why this boilerplate is needed
impl <'a> ResourceClient<'a, Dataset> for DatasetCollectionClient<'a> {
    fn get_client(&self) -> &'a ApifyClient {
        self.apify_client
    }

    fn get_url_segment(&self) -> &str {
        &self.url_segment
    }
}

impl <'a> DatasetCollectionClient<'a> {
    pub fn new(apify_client: &'a ApifyClient, identifier: &str) -> Self {
        DatasetCollectionClient {
            apify_client,
            url_segment: "dataset".to_owned(),
        }
    }
}