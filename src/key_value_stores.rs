use crate::client::ApifyClient;

struct Record {
}

pub async fn get_record(
    client: &ApifyClient,
    store_id: &str,
    key: &str
) {
    let url = format!("{}/key-value-stores/{}/records/{}", client.base_path, store_id, key);
    // client.client.get(url).await?;
}