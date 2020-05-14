use crate::client::{ApifyClient, ApifyClientError, IdOrName, PaginationList};
use crate::utils::{create_resource_locator, ResourceType};

use querystring;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct ApifyClientResult<T> {
    data: T
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Dataset {
    id: String,
    name: Option<String>,
    user_id: String,
    created_at: String,
    modified_at: String,
    accessed_at: String,
    item_count: u32,
    clean_item_count: Option<u32>,
    act_id: Option<String>,
    act_run_id: Option<String>
}

pub struct ListDatasetsParams {
    offset: Option<u32>,
    limit: Option<u32>,
    desc: Option<bool>,
    unnamed: Option<bool>,
}

impl ApifyClient {
    /// Gets a dataset info object
    /// If you provide dataset ID, you don't need a token
    /// If you provide username~datasetName, you need a token (otherwise it will panic)
    pub async fn get_dataset(&self, dataset_id_or_name: &IdOrName) -> Result<Dataset, ApifyClientError> {
        let dataset_id_or_name_val = create_resource_locator(self, dataset_id_or_name, ResourceType::Dataset);
        let url = format!("{}/datasets/{}", self.base_path, dataset_id_or_name_val);
        let url_with_query = match &self.optional_token {
            None => url,
            Some(token) => format!("{}?token={}", &url, token)
        };
        println!("Constructed URL: {}", url_with_query);
        let headers = reqwest::header::HeaderMap::new();
        let resp = self.retrying_request(&url_with_query, &reqwest::Method::GET, vec![], headers).await;
        match resp {
            Err(err) => Err(err),
            Ok(raw_data) => { 
                let apify_client_result: ApifyClientResult<Dataset> = raw_data.json().await.unwrap();
                return Ok(apify_client_result.data);
            }
        }
    }

    /// List datasets of the provided account
    /// Requires a token
    pub async fn list_datasets(&self, optional_params: Option<ListDatasetsParams>) -> PaginationList<Dataset> {
        unimplemented!()
    }
}