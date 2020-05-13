use crate::client::{ApifyClient, ApifyClientError};
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

impl ApifyClient {
    pub async fn get_dataset(&self, dataset_id: &str) -> Result<Dataset,ApifyClientError> {
        let url = format!("{}/datasets/{}", self.base_path, dataset_id);
        let headers = reqwest::header::HeaderMap::new();
        let resp = self.retrying_request(&url, &reqwest::Method::GET, vec![], headers).await;
        match resp {
            Err(err) => Err(err),
            Ok(raw_data) => { 
                let apify_client_result: ApifyClientResult<Dataset> = raw_data.json().await.unwrap();
                return Ok(apify_client_result.data);
            }
        }
    }
}