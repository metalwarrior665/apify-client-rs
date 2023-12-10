use crate::generic_types::{BaseBuilder, NoContent};
use std::marker::PhantomData;
use crate::apify_client::ApifyClient;

// NOTE: Would be cool if we could use traits as lightweight inheritance by forcing the implementer
// to be a struct with certain fields and there is a proposal
// for the Rust lang there but it is kinda dead https://github.com/rust-lang/rfcs/pull/1546
// Until that is done, we need individual resource client to implement getters to their properties
pub trait ResourceClient<'a, T> {
    fn get_client(&self) -> &'a ApifyClient;
    fn get_url_segment(&self) -> &str;
    fn get_identifier(&self) -> &str;

    fn get(&self) -> BaseBuilder<'a, T> {
        BaseBuilder::new(
            self.get_client(),
            self.get_url_segment().to_owned(),
            self.get_identifier().to_owned(), 
            reqwest::Method::GET
        )
    }

    fn update(&self, body:  Result<Option<Vec<u8>>, serde_json::Error>) -> BaseBuilder<'a, T> {
        let mut builder = BaseBuilder::new(
            self.get_client(),
            self.get_url_segment().to_owned(),
            self.get_identifier().to_owned(),
            reqwest::Method::PUT,
        );
        builder.body(body);
        builder
    }

    fn delete(&self) -> BaseBuilder<'a, NoContent> {
        BaseBuilder::new(
            self.get_client(),
            self.get_url_segment().to_owned(),
            self.get_identifier().to_owned(),
            reqwest::Method::DELETE,
        )
    }
}