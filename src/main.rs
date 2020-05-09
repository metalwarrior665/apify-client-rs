extern crate reqwest;
extern crate serde;
extern crate tokio;

mod client;
mod key_value_stores;
mod datasets;

fn main() {
    let apifyClient = client::ApifyClient::new("hello".to_owned());
}
