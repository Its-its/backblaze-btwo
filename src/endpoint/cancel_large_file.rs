use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{AccountAuthorization, BackblazeResponseError, FileId, Result};

/// https://www.backblaze.com/b2/docs/b2_cancel_large_file.html
pub async fn cancel_large_file(
    file_id: &FileId,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<CancelLargeFileResponse> {
    let body = json!({
        "fileId": file_id,
    });

    let resp = client
        .post(format!("{}/b2api/v2/b2_cancel_large_file", auth.api_url).as_str())
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelLargeFileResponse {
    pub file_id: String,
    pub account_id: String,
    pub bucket_id: String,
    pub file_name: String,
}
