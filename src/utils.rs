use crate::client::{ ApifyClient, IdOrName };
use reqwest::header::{HeaderMap, CONTENT_TYPE};

#[derive(Debug)]
pub enum ResourceType {
    Dataset,
}

// Creates a string represantion of a resource on the Apify platform via API
pub fn create_resource_locator (client: &ApifyClient, id_or_name: &IdOrName, resource_type: ResourceType) -> String {
    match id_or_name {
        IdOrName::Id(id) => String::from(id),
        IdOrName::Name(name) => {
            if client.optional_token.is_none() {
                panic!("Using {:?} name requires a token!", resource_type);
            }
            format!("{}~{}", name.user_name_or_user_id, name.resource_name)
        }
    }
}

pub fn json_content_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}