use crate::client::{ ApifyApiError, IdOrName };
use reqwest::header::{HeaderMap, CONTENT_TYPE};

#[derive(Debug)]
pub enum ResourceType {
    Dataset,
}

// Creates a string represantion of a resource on the Apify platform via API
pub fn stringify_resource (id_or_name: &IdOrName) -> String {
    match id_or_name {
        IdOrName::Id(id) => String::from(id),
        IdOrName::Name(name) => format!("{}~{}", name.user_name_or_user_id, name.resource_name)
    }
}

pub fn is_resource_by_name(id_or_name: &IdOrName) -> bool {
    if let IdOrName::Name(_) = id_or_name {
        return true;
    }
    return false;
}

pub fn json_content_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers
}

pub fn parse_pagination_header(headers: &HeaderMap, header_name: &str) -> Result<u64, ApifyApiError> {
    headers
        .get(header_name)
        .ok_or(ApifyApiError::ApiFailure(format!("{} headers missing in response!", header_name)))?
        .to_str()
        .map_err(|_| ApifyApiError::ApiFailure(format!("{} header is not valid UTF-8!", header_name)))?
        .parse()
        .map_err(|_| ApifyApiError::ApiFailure(format!("{} header cannot be parsed to a u64!", header_name)))
}