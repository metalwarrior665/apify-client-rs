extern crate reqwest;
extern crate serde;
extern crate tokio;

mod client;
mod key_value_stores;
mod datasets;

#[tokio::main]
async fn main() {
    let token = std::env::var("APIFY_TOKEN").unwrap();
    let my_client = client::ApifyClient::new(token);
    let dataset = my_client.get_dataset("h6HQ8i1ea1BthSQpf").await;
    println!("{:?}", dataset);
}
