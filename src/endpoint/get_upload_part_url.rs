use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AccountAuthorization, BackblazeResponseError, FileId, Result};

/// https://www.backblaze.com/b2/docs/b2_get_upload_part_url.html
pub async fn get_upload_part_url(
    file_id: &FileId,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<UploadPartUrlResponse> {
    let body = json!({
        "fileId": file_id,
    });

    let resp = client
        .post(format!("{}/b2api/v2/b2_get_upload_part_url", auth.api_url).as_str())
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
pub struct UploadPartUrlResponse {
    pub file_id: String,
    pub upload_url: String,
    pub authorization_token: String,
}
