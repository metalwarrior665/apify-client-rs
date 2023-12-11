use crate::generic_types::{BaseBuilder, NoOutput};
use crate::apify_client::ApifyClient;

pub trait ResourceCollectionClient<'a, T> {
    fn get_client(&self) -> &'a ApifyClient;
    fn get_url_segment(&self) -> &str;

    fn list(&self) -> BaseBuilder<'a, T> {
        BaseBuilder::new(
            self.get_client(),
            self.get_url_segment().to_owned(),
            reqwest::Method::GET
        )
    }
}