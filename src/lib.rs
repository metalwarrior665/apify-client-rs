extern crate reqwest;
extern crate serde;
extern crate tokio;
#[macro_use]
extern crate query_params;
#[macro_use]
extern crate serde_json;

mod client;
mod key_value_stores;
mod datasets;
mod request;
mod utils;
mod generic_types;

use crate::client::{ApifyClient, IdOrName};