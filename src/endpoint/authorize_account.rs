use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    ApplicationKeyId, BackblazeResponseError, BucketId, Capability, Credentials, Result,
    util::API_URL_V4,
};

/// https://www.backblaze.com/b2/docs/b2_authorize_account.html
pub async fn authorize_account(
    creds: &Credentials,
    client: &Client,
) -> Result<AccountAuthorization> {
    let resp = client
        .get(format!("{API_URL_V4}/b2_authorize_account").as_str())
        .header(creds.header_name(), creds.auth_string())
        .send()
        .await?;

    if resp.status().as_u16() == 200 {
        Ok(resp.json().await?)
    } else {
        Err(resp.json::<BackblazeResponseError>().await?.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountAuthorization {
    pub account_id: ApplicationKeyId,
    pub(crate) authorization_token: String,
    // pub(create) application_key_expiration_timestamp: Option<i64>,
    pub api_info: ApiInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiInfo {
    // TODO: Both of these may be optional, if so we may want to use an enum instead.
    pub groups_api: Option<GroupsApi>,
    pub storage_api: StorageApi,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupsApi {
    pub capabilities: Vec<String>,
    pub groups_api_url: String,
    pub info_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageApi {
    pub absolute_minimum_part_size: usize,
    pub api_url: String,
    // allowed: Object,
    pub download_url: String,
    pub recommended_part_size: usize,
    pub s3_api_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizeAccountAllowed {
    pub capabilities: Vec<Capability>,
    pub bucket_id: Option<BucketId>,
    pub bucket_name: Option<String>,
    pub name_prefix: Option<String>,
}
