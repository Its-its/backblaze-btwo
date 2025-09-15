use reqwest::Client;
use serde_json::json;

use crate::{AccountAuthorization, BackblazeResponseError, FileId, Result};

use super::UploadFileResponse;

/// https://www.backblaze.com/b2/docs/b2_finish_large_file.html
pub async fn finish_large_file(
    file_id: &FileId,
    ordered_sha1_parts: &[String],
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<UploadFileResponse> {
    let body = json!({
        "fileId": file_id,
        "partSha1Array": ordered_sha1_parts,
    });

    let resp = client
        .post(format!("{}/b2api/v2/b2_finish_large_file", auth.api_url).as_str())
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
