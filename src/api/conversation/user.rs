use crate::types::pages::{Links, PageMeta};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct UserList {
    users: Vec<User>,
}

#[derive(Debug, Deserialize)]
pub struct UserListPage {
    meta: PageMeta,
    users: Vec<User>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(skip_serializing)] // This field should not be serialized
    pub id: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<UserProperties>,
    #[serde(skip_serializing)] // This field should not be serialized
    pub _links: Option<Links>,
}

impl User {
    pub fn new(name: String) -> Self {
        User {
            id: None,
            name,
            display_name: None,
            image_url: None,
            properties: None,
            _links: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_sort_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    custom_data: Option<HashMap<String, Value>>,
}
