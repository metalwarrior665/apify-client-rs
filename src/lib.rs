#[macro_use]
extern crate query_params;
#[macro_use]
extern crate serde_json;

pub mod apify_client;
// pub mod datasets;
pub mod http_request;
pub mod utils;
pub mod generic_types;
pub mod error;
pub mod resource_clients;
pub mod base_clients;
pub mod builders;


// These are integration tests that call Apify APIs
// They require an API token in test/test_token.txt file as plain string
// TODO: Cleanup if tests crash in the middle
#[cfg(test)]
mod test {
    use super::apify_client::ApifyClient;
    use super::error::{ApifyApiError, ApifyClientError};
    use super::generic_types::{NoOutput, PaginationList};
    use serde::{Serialize, Deserialize};
    use super::resource_clients::run::Run;
    use super::resource_clients::dataset::Dataset;
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

    
    fn create_dataset (client: &ApifyClient, name: &str) -> Dataset {
        unimplemented!();
        //let dataset = await_test!(client.create_dataset(name).send()).unwrap();
        //dataset
    }

    fn update_dataset (client: &ApifyClient, id_or_name: &str, name: &str) -> Dataset {
        let dataset = await_test!(client.dataset(id_or_name).update(name).send()).unwrap();
        dataset
    }

    fn delete_dataset (client: &ApifyClient, id_or_name: &str) -> NoOutput {
        let no_content = await_test!(client.dataset(id_or_name).delete().send()).unwrap();
        no_content
    }

    fn push_items (client: &ApifyClient, id_or_name: &str, items: Vec<Item>) -> Result<NoOutput, ApifyClientError> {
        let put_result = await_test!(client.dataset(id_or_name).push_items(&items).send());
        put_result
    }

    fn list_items (client: &ApifyClient, id_or_name: &str) -> Result<PaginationList<Item>, ApifyClientError> {
        let maybe_pagination_list = await_test!(client.dataset(id_or_name).list_items().send());
        maybe_pagination_list
    }
 
    fn download_items (client: &ApifyClient, id_or_name: &str) -> Result<String, ApifyClientError> {
        let maybe_string = await_test!(client.dataset(id_or_name).download_items(crate::builders::dataset::Format::Csv).send());
        match maybe_string {
            Ok(bytes) => Ok(String::from_utf8(bytes).unwrap()),
            Err(err) => Err(err),
        }
    }
    

    fn get_run (client: &ApifyClient, id_or_name: &str) -> Result<Run, ApifyClientError> {
        let maybe_run = await_test!(client.run(id_or_name).get().send());
        maybe_run
    }

    fn get_dataset (client: &ApifyClient, id_or_name: &str) -> Result<super::resource_clients::dataset::Dataset, ApifyClientError> {
        let maybe_dataset = await_test!(client.dataset(id_or_name).get().send());
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

        let dataset_id = dataset.id;

        let maybe_dataset = get_dataset(&client, &dataset_id);
        assert_eq!(maybe_dataset.unwrap().name.unwrap(), name);

        let new_name = "RUST-TEST-UPDATE";
        let dataset = update_dataset(&client, &dataset_id, new_name);
        assert_eq!(dataset.name.unwrap(), new_name);

        let maybe_dataset = get_dataset(&client, &dataset_id);
        assert_eq!(maybe_dataset.unwrap().name.unwrap(), new_name);

        let no_content = delete_dataset(&client, &dataset.id);
        assert_eq!(no_content, NoOutput::new());

        let maybe_dataset = get_dataset(&client, &dataset_id);
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
        
        /* 
        let maybe_pagination_list = await_test!(client.list_datasets().limit(10).send());
        assert!(maybe_pagination_list.is_ok());
        assert!(maybe_pagination_list.unwrap().items.iter().find(|dataset| dataset.id == dataset_id.clone()).is_some());

        delete_dataset(&client, dataset_id.clone());

        let maybe_pagination_list = await_test!(client.list_datasets().limit(10).send());
        assert!(maybe_pagination_list.is_ok());
        assert!(maybe_pagination_list.unwrap().items.iter().find(|dataset| dataset.id == dataset_id).is_none());
        */
    }

    // TODO: Test all formats and most params
    #[test] 
    fn put_get_items_test () {
        let client = create_client();
        let name = "RUST-TEST-PUT-ITEMS";

        let dataset = create_dataset(&client, name);
        let dataset_id = dataset.id;

        let items = get_test_items();
        let put_result = push_items(&client, &dataset_id, items.clone());
        println!("{:?}", put_result);
        assert!(put_result.is_ok());
        assert_eq!(put_result.unwrap(), NoOutput::new());

        // We have to sleep so that numbers on Apify's side update propagate properly
        std::thread::sleep(std::time::Duration::from_secs(10));

        let maybe_pagination_list = list_items(&client, &dataset_id);
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

        let maybe_string = download_items(&client, &dataset_id);
        
        let no_content = delete_dataset(&client, &dataset_id);

        assert_eq!(pagination_list, pagination_list_test);
        // We need to assert here so that we delete the dataset
        assert!(maybe_string.is_ok());
        println!("{}", maybe_string.unwrap());

        assert_eq!(no_content, NoOutput::new());
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