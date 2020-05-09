use crate::client::ApifyClient;

struct Dataset {
    id: String,
    name: String,
    user_id: String,
    created_at: String,
    modified_at: String,
    accessed_at: String,
    item_count: u32,
    clean_item_count: u32
}

impl ApifyClient {
    fn getDataset(&self, datasetId: &str) -> Dataset {
        
    }
}