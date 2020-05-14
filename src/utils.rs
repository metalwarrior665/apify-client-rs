use crate::client::{ ApifyClient, IdOrName };

#[derive(Debug)]
pub enum ResourceType {
    Dataset,
}

pub fn create_resource_locator (client: &ApifyClient, id_or_name: &IdOrName, resource_type: ResourceType) -> String {
    match id_or_name {
        IdOrName::Id(id) => String::from(id),
        IdOrName::Name(dataset_name) => {
            if client.optional_token.is_none() {
                panic!("Using {:?} name requires a token!", resource_type);
            }
            format!("{}~{}", dataset_name.user_name_or_user_id, dataset_name.resource_name)
        }
    }
}