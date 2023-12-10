#[macro_use]
extern crate query_params;
#[macro_use]
extern crate serde_json;

pub mod apify_client;
pub mod datasets;
pub mod http_request;
pub mod utils;
pub mod generic_types;
pub mod error;
pub mod resource_clients;
pub mod base_clients;


// These are integration tests that call Apify APIs
// They require an API token in test/test_token.txt file as plain string
// TODO: Cleanup if tests crash in the middle
#[cfg(test)]
mod test {
    use super::apify_client::ApifyClient;
    use super::error::{ApifyApiError, ApifyClientError};
    use super::datasets::Dataset;
    use super::generic_types::{NoContent, PaginationList, IdOrName};
    use serde::{Serialize, Deserialize};
    use super::resource_clients::run::Run;
    use super::base_clients::resource_client::ResourceClient;

    // Simple await macro for tests
    macro_rules! await_test {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Item {
        field1: f64,
        field2: f64,
    }
    fn get_test_items() -> Vec<Item> {
        vec![Item { field1: 1., field2: 2. }, Item { field1: 3., field2: 4. }]
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

    fn put_items (client: &ApifyClient, id_or_name: &IdOrName, items: Vec<Item>) -> Result<NoContent, ApifyClientError> {
        let put_result = await_test!(client.put_items(id_or_name, &items).send());
        put_result
    }


    fn get_items (client: &ApifyClient, id_or_name: IdOrName) -> Result<PaginationList<Item>, ApifyClientError> {
        let maybe_pagination_list = await_test!(client.get_items(id_or_name).send());
        maybe_pagination_list
    }

    fn get_items_raw_csv (client: &ApifyClient, id_or_name: IdOrName) -> Result<String, ApifyClientError> {
        let maybe_string = await_test!(client.get_items_raw(id_or_name).format(crate::datasets::Format::Csv).send());
        maybe_string
    }

    fn get_run (client: &ApifyClient, id_or_name: &str) -> Result<Run, ApifyClientError> {
        let maybe_run = await_test!(client.run(id_or_name).get().send());
        maybe_run
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
        let is_correct_error = match maybe_dataset.unwrap_err() {
            ApifyClientError::ApifyApi(ApifyApiError::NotFound(text)) => text == "Dataset was not found".to_string(),
            _ => false,
        };
        assert!(is_correct_error);
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

    // TODO: Test all formats and most params
    #[test] 
    fn put_get_items_test () {
        let client = create_client();
        let name = "RUST-TEST-PUT-ITEMS";

        let dataset = create_dataset(&client, name);
        let dataset_id = dataset.id;

        let items = get_test_items();
        let put_result = put_items(&client, &IdOrName::Id(dataset_id.clone()), items.clone());
        println!("{:?}", put_result);
        assert!(put_result.is_ok());
        assert_eq!(put_result.unwrap(), NoContent::new());

        // We have to sleep so that numbers on Apify's side update propagate properly
        std::thread::sleep(std::time::Duration::from_secs(10));

        let maybe_pagination_list = get_items(&client, IdOrName::Id(dataset_id.clone()));
        assert!(maybe_pagination_list.is_ok());
        let pagination_list = maybe_pagination_list.unwrap();
        println!("{:?}", pagination_list);
        let pagination_list_test = PaginationList{
            total: 2,
            offset: 0,
            limit: Some(999999999999),
            count: 2,
            desc: false,
            items: get_test_items(),
        };

        let maybe_string = get_items_raw_csv(&client, IdOrName::Id(dataset_id.clone()));
        
        let no_content = delete_dataset(&client, &IdOrName::Id(dataset_id.clone()));

        assert_eq!(pagination_list, pagination_list_test);
        // We need to assert here so that we delete the dataset
        assert!(maybe_string.is_ok());
        println!("{}", maybe_string.unwrap());

        assert_eq!(no_content, NoContent::new());
    }

    #[test]
    fn get_run_test () {
        let client = create_client();
        // TODO: unhardcode the ID 
        let maybe_run = get_run(&client, "D7mahEK1QsWkUJ1Py");
        println!("maybe run {:?}", maybe_run);
        assert!(maybe_run.is_ok());
        let run = maybe_run.unwrap();
        assert_eq!(run.meta.origin, "DEVELOPMENT");
    }
}