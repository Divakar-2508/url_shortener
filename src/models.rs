use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UrlPayload {
    pub url: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct UrlData {
    pub id: u32,
    pub url: String,
    pub short_code: String,
    pub created_at: String,
    pub updated_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_count: Option<u32>,
}
