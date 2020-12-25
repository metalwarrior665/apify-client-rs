#[macro_use]
extern crate query_params;
#[macro_use]
extern crate serde_json;

pub mod client;
pub mod key_value_stores;
pub mod datasets;
pub mod request;
pub mod utils;
pub mod generic_types;

use crate::client::{ApifyClient, IdOrName, ApifyClientError};
use crate::datasets::{Dataset};
use crate::generic_types::{NoContent};

// These are integration tests that call Apify APIs
// They require an API token in test/test_token.txt file as plain string
#[cfg(test)]
mod test {
    use super::*;

    // Simple await macro for tests
    macro_rules! await_test {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    // You must have token in test/test_token.txt file as plain string
    fn create_client () -> ApifyClient {
        let path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        let token = std::fs::read_to_string("test/test_token.txt");
        println!("{:?}", token);
        let my_client = ApifyClient::new(token.ok());
        my_client
    }    

    // Helper functions for sending the actual requests
    // Needed because we need to clean up after each test
    fn create_dataset (client: &ApifyClient, name: &str) -> Dataset {
        let dataset = await_test!(client.create_dataset(name).send()).unwrap();
        dataset
    }

    fn update_dataset (client: &ApifyClient, id_or_name: &IdOrName, name: &str) -> Dataset {
        let dataset = await_test!(client.update_dataset(id_or_name, name).send()).unwrap();
        dataset
    }

    fn delete_dataset (client: &ApifyClient, id_or_name: &IdOrName) -> NoContent {
        let no_content = await_test!(client.delete_dataset(id_or_name).send()).unwrap();
        no_content
    }

    fn get_dataset (client: &ApifyClient, id_or_name: &IdOrName) -> Result<Dataset, ApifyClientError> {
        let maybe_dataset = await_test!(client.get_dataset(id_or_name).send());
        maybe_dataset
    }

    // This is done as one mega test to limit number of API calls when cleaning
    // but perhaps there is a better way
    #[test]
    fn create_update_get_and_delete_dataset () {
        let client = create_client();
        let name = "RUST-TEST-CREATE";

        let dataset = create_dataset(&client, name);
        assert_eq!(dataset.name.unwrap(), name);

        let dataset_id = dataset.id.clone();

        let maybe_dataset = get_dataset(&client, &IdOrName::Id(dataset_id.clone()));
        assert_eq!(maybe_dataset.unwrap().name.unwrap(), name);

        let new_name = "RUST-TEST-UPDATE";
        let dataset = update_dataset(&client, &IdOrName::Id(dataset_id.clone()), new_name);
        assert_eq!(dataset.name.unwrap(), new_name);

        let maybe_dataset = get_dataset(&client, &IdOrName::Id(dataset_id.clone()));
        assert_eq!(maybe_dataset.unwrap().name.unwrap(), new_name);

        let no_content = delete_dataset(&client, &IdOrName::Id(dataset.id));
        assert_eq!(no_content, NoContent::new());

        let maybe_dataset = get_dataset(&client, &IdOrName::Id(dataset_id));
        assert!(maybe_dataset.is_err());
        assert_eq!(maybe_dataset.unwrap_err(), ApifyClientError::NotFound("Dataset was not found".to_string()));
    }
    
    #[test]
    fn list_datasets_test () {
        let client = create_client();
        let name = "RUST-TEST-LIST";

        let dataset = create_dataset(&client, name);
        let dataset_id = dataset.id;
        
        let maybe_pagination_list = await_test!(client.list_datasets().limit(10).send());
        assert!(maybe_pagination_list.is_ok());
        assert!(maybe_pagination_list.unwrap().items.iter().find(|dataset| dataset.id == dataset_id.clone()).is_some());

        delete_dataset(&client, &IdOrName::Id(dataset_id.clone()));

        let maybe_pagination_list = await_test!(client.list_datasets().limit(10).send());
        assert!(maybe_pagination_list.is_ok());
        assert!(maybe_pagination_list.unwrap().items.iter().find(|dataset| dataset.id == dataset_id).is_none());
    }

    // TODO: Add get items into the test
    #[test] 
    fn put_items () {
        let client = create_client();
        let name = "RUST-TEST-PUT-ITEMS";

        let dataset = create_dataset(&client, name);
        let dataset_id = dataset.id;

        let item1 = serde_json::json!({ "obj": 1 });
        let item2 = serde_json::json!({ "obj": 2 });
        let v = vec![item1, item2];
        let put_result = await_test!(client.put_items(&IdOrName::Id(dataset_id.clone()), &v).send());
        assert!(put_result.is_ok());
        assert_eq!(put_result.unwrap(), NoContent::new());

        let no_content = delete_dataset(&client, &IdOrName::Id(dataset_id.clone()));
        assert_eq!(no_content, NoContent::new());
    }
}