use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    util::API_URL_V2, ApplicationKeyId, BackblazeResponseError, BucketId, Capability, Credentials,
    Result,
};

/// https://www.backblaze.com/b2/docs/b2_authorize_account.html
pub async fn authorize_account(
    creds: &Credentials,
    client: &Client,
) -> Result<AccountAuthorization> {
    let resp = client
        .get(format!("{API_URL_V2}/b2_authorize_account").as_str())
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
    // allowed: Object,
    pub api_url: String,
    pub download_url: String,
    pub recommended_part_size: usize,
    pub absolute_minimum_part_size: usize,
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
