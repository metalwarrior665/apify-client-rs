extern crate reqwest;
extern crate serde;
extern crate tokio;
#[macro_use]
extern crate query_params;

mod client;
mod key_value_stores;
mod datasets;
mod request;
mod utils;
mod generic_types;

use crate::client::{ IdOrName, ResourceName };

#[tokio::main]
async fn main() {
    let token = std::env::var("APIFY_TOKEN");
    let my_client = client::ApifyClient::new(token.ok());

    // CREATE DATASET
    
    let name = "RUST-TEST";
    let dataset = my_client.create_dataset(name).send().await;
    println!("{:?}", dataset);
    
    let dataset_id = IdOrName::Id(dataset.unwrap().id);

    // DELETE DATASET
    /*
    my_client.delete_dataset(&dataset_id).send().await;
    */

    // PUT ITEMS

    let item1 = serde_json::json!({ "obj": 1 });
    let item2 = serde_json::json!({ "obj": 2 });
    let v = vec![item1, item2];
    let put_result = my_client.put_items(&dataset_id, &v).send().await;
    println!("Put result: {:?}", put_result);

    // LIST DATASETS
    /*
    let resource_name = ResourceName { user_name_or_user_id: "gdgdf".to_string(), resource_name: "flightclub-unfulfilled".to_string() };
    let resource = IdOrName::Name(resource_name);
    let dataset = my_client.list_datasets().unnamed(true).limit(25).send().await;
    println!("{:?}", dataset);
    */
}
