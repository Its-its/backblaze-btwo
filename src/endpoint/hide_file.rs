use reqwest::Client;
use serde_json::json;

use crate::{encode_file_name, AccountAuthorization, BackblazeResponseError, BucketId, Result};

/// https://www.backblaze.com/b2/docs/b2_hide_file.html
pub async fn hide_file(
    bucket_id: &BucketId,
    file_path: &str,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<serde_json::Value> {
    let body = json!({
        "bucketId": bucket_id,
        "fileName": encode_file_name(file_path)
    });

    let resp = client
        .post(format!("{}/b2api/v2/b2_hide_file", auth.api_url).as_str())
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
