extern crate reqwest;
extern crate serde;
extern crate tokio;
extern crate querystring;

mod client;
mod key_value_stores;
mod datasets;
mod request;
mod utils;

use crate::client::{ IdOrName, ResourceName };

#[tokio::main]
async fn main() {
    let token = std::env::var("APIFY_TOKEN");
    let my_client = client::ApifyClient::new(token.ok());
    let resource_name = ResourceName { user_name_or_user_id: "gdgdf".to_string(), resource_name: "flightclub-unfulfilled".to_string() };
    let resource = IdOrName::Name(resource_name);
    let dataset = my_client.get_dataset(&resource).await;
    println!("{:?}", dataset);
}
