use crate::apify_client::ApifyClient;
use serde::Deserialize;
use crate::base_clients::resource_client::ResourceClient;

pub struct RunClient<'a> {
    apify_client: &'a ApifyClient,
    url_segment: String,
    identifier: String,
}

// See comment on the ResourceClient trait why this boilerplate is needed
impl <'a> ResourceClient<'a, Run> for RunClient<'a> {
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

impl <'a> RunClient<'a> {
    pub fn new(apify_client: &'a ApifyClient, identifier: &str) -> Self {
        RunClient {
            apify_client,
            url_segment: "actor-runs".to_owned(),
            identifier: identifier.to_owned(),
        }
    }
}

// Annoying types at bottom
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Run {
    pub id: String,
    #[serde(rename = "actId")] 
    pub actor_id: String,
    pub user_id: String,
    pub actor_task_id: Option<String>,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub status: String,
    pub status_message: Option<String>,
    pub is_status_message_terminal: Option<bool>,
    pub meta: Meta,
    pub stats: Stats,
    pub options: Options,
    pub build_id: String,
    pub exit_code: u64,
    pub default_key_value_store_id: String,
    pub default_dataset_id: String,
    pub default_request_queue_id: String,
    pub build_number: String,
    pub container_url: String,
    pub is_container_server_ready: Option<bool>,
    pub git_branch_name: Option<String>,
    pub usage: Usage,
    pub usage_total_usd: f64,
    pub usage_usd: UsageUsd,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub origin: String,
    // Only when run was started via API
    pub client_ip: Option<String>,
    pub user_agent: String,
    
}
#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub input_body_len: u64,
    pub reboot_count: u32,
    pub restart_count: u32,
    pub duration_millis: u64,
    pub resurrect_count: u32,
    pub mem_avg_bytes: f64,
    pub mem_max_bytes: u64,
    pub mem_current_bytes: u64,
    pub cpu_avg_usage: f64,
    pub cpu_max_usage: f64,
    pub cpu_current_usage: u64,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub run_time_secs: f64,
    pub metamorph: u64,
    pub compute_units: f64,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    pub build: String,
    pub timeout_secs: u64,
    pub memory_mbytes: u32,
    pub disk_mbytes: u32,
}

#[derive(Default, Debug, Clone,  Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    #[serde(rename = "ACTOR_COMPUTE_UNITS")]
    pub actor_compute_units: f64,
    #[serde(rename = "DATASET_READS")]
    pub dataset_reads: u64,
    #[serde(rename = "DATASET_WRITES")]
    pub dataset_writes: u64,
    #[serde(rename = "KEY_VALUE_STORE_READS")]
    pub key_value_store_reads: u64,
    #[serde(rename = "KEY_VALUE_STORE_WRITES")]
    pub key_value_store_writes: u64,
    #[serde(rename = "KEY_VALUE_STORE_LISTS")]
    pub key_value_store_lists: u64,
    #[serde(rename = "REQUEST_QUEUE_READS")]
    pub request_queue_reads: u64,
    #[serde(rename = "REQUEST_QUEUE_WRITES")]
    pub request_queue_writes: u64,
    #[serde(rename = "DATA_TRANSFER_INTERNAL_GBYTES")]
    pub data_transfer_internal_gbytes: f64,
    #[serde(rename = "DATA_TRANSFER_EXTERNAL_GBYTES")]
    pub data_transfer_external_gbytes: f64,
    #[serde(rename = "PROXY_RESIDENTIAL_TRANSFER_GBYTES")]
    pub proxy_residential_transfer_gbytes: f64,
    #[serde(rename = "PROXY_SERPS")]
    pub proxy_serps: u64,
}

#[derive(Default, Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UsageUsd {
    #[serde(rename = "ACTOR_COMPUTE_UNITS")]
    pub actor_compute_units: f64,
    #[serde(rename = "DATASET_READS")]
    pub dataset_reads: f64,
    #[serde(rename = "DATASET_WRITES")]
    pub dataset_writes: f64,
    #[serde(rename = "KEY_VALUE_STORE_READS")]
    pub key_value_store_reads: f64,
    #[serde(rename = "KEY_VALUE_STORE_WRITES")]
    pub key_value_store_writes: f64,
    #[serde(rename = "KEY_VALUE_STORE_LISTS")]
    pub key_value_store_lists: f64,
    #[serde(rename = "REQUEST_QUEUE_READS")]
    pub request_queue_reads: f64,
    #[serde(rename = "REQUEST_QUEUE_WRITES")]
    pub request_queue_writes: f64,
    #[serde(rename = "DATA_TRANSFER_INTERNAL_GBYTES")]
    pub data_transfer_internal_gbytes: f64,
    #[serde(rename = "DATA_TRANSFER_EXTERNAL_GBYTES")]
    pub data_transfer_external_gbytes: f64,
    #[serde(rename = "PROXY_RESIDENTIAL_TRANSFER_GBYTES")]
    pub proxy_residential_transfer_gbytes: f64,
    #[serde(rename = "PROXY_SERPS")]
    pub proxy_serps: f64,
}