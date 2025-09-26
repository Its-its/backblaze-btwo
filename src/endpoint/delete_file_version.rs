use reqwest::Client;
use serde_json::json;

use crate::{
    AccountAuthorization, BackblazeResponseError, FileId, Result, encode_file_name,
    endpoint::DeleteFileVersionResponse,
};

/// https://www.backblaze.com/b2/docs/b2_delete_file_version.html
pub async fn delete_file_version(
    file_path: &str,
    file_id: FileId,
    bypass_governance: bool,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<DeleteFileVersionResponse> {
    let body = json!({
        "fileName": encode_file_name(file_path),
        "fileId": file_id,
        "bypassGovernance": bypass_governance,
    });

    let resp = client
        .post(
            format!(
                "{}/b2api/v4/b2_delete_file_version",
                auth.api_info.storage_api.api_url
            )
            .as_str(),
        )
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
