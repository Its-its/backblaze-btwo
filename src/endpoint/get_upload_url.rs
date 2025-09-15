use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AccountAuthorization, BackblazeResponseError, BucketId, Result};

pub async fn get_upload_url(
    bucket_id: &BucketId,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<UploadUrlResponse> {
    let body = json!({ "bucketId": bucket_id });

    let resp = client
        .post(format!("{}/b2api/v2/b2_get_upload_url", auth.api_url).as_str())
        .header("Authorization", auth.authorization_token.as_str())
        .body(serde_json::to_string(&body)?)
        .send()
        .await?;

    if resp.status().as_u16() == 200 {
        Ok(resp.json().await?)
    } else {
        Err(resp.json::<BackblazeResponseError>().await?.into())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadUrlResponse {
    pub authorization_token: String,
    pub bucket_id: String,
    pub upload_url: String,
}
