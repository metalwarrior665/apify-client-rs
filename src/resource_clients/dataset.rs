use crate::apify_client::ApifyClient;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use crate::base_clients::resource_client::ResourceClient;
use crate::generic_types::{BaseBuilder, PaginationList};
use crate::error::ApifyClientError;
use crate::builders::dataset::GetItemsBuilder;
use std::marker::PhantomData;

pub struct DatasetClient<'a> {
    pub apify_client: &'a ApifyClient,
    pub url_segment: String,
    pub identifier: String,
}

// See comment on the ResourceClient trait why this boilerplate is needed
impl <'a> ResourceClient<'a, Dataset> for DatasetClient<'a> {
    fn get_client(&self) -> &'a ApifyClient {
        self.apify_client
    }

    fn get_url_segment(&self) -> &str {
        &self.url_segment
    }

    fn get_identifier(&self) -> &str {
        &self.identifier
    }
}

impl <'a> DatasetClient<'a> {
    pub fn new(apify_client: &'a ApifyClient, identifier: &str) -> Self {
        DatasetClient {
            apify_client,
            url_segment: "dataset".to_owned(),
            identifier: identifier.to_owned(),
        }
    }

    pub fn list_items<T: serde::de::DeserializeOwned>(&self) -> GetItemsBuilder<T> {
        GetItemsBuilder::new(self)
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
