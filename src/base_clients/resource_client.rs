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
        BaseBuilder {
            client: self.get_client(),
            url_segment: self.get_url_segment().to_owned(),
            identifier: self.get_identifier().to_owned(),
            method: reqwest::Method::GET,
            body: Ok(None),
            phantom: PhantomData
        }
    }

    fn update(&self, body:  Result<Option<Vec<u8>>, serde_json::Error>) -> BaseBuilder<'a, T> {
        BaseBuilder {
            client: self.get_client(),
            url_segment: self.get_url_segment().to_owned(),
            identifier: self.get_identifier().to_owned(),
            method: reqwest::Method::PUT,
            body,
            phantom: PhantomData
        }
    }

    fn delete(&self) -> BaseBuilder<'a, NoContent> {
        BaseBuilder {
            client: self.get_client(),
            url_segment: self.get_url_segment().to_owned(),
            identifier: self.get_identifier().to_owned(),
            method: reqwest::Method::DELETE,
            body: Ok(None),
            phantom: PhantomData
        }
    }
}