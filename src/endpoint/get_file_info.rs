use reqwest::Client;

use crate::{
    AccountAuthorization, BackblazeResponseError, FileId, Result, endpoint::GetFileInfoResponse,
};

/// https://www.backblaze.com/b2/docs/b2_get_file_info.html
pub async fn get_file_info(
    id: FileId,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<GetFileInfoResponse> {
    let resp = client
        .get(
            format!(
                "{}/b2api/v4/b2_get_file_info?field={id}",
                auth.api_info.storage_api.api_url
            )
            .as_str(),
        )
        .header("Authorization", auth.authorization_token.as_str())
        .send()
        .await?;

    if resp.status().as_u16() == 200 {
        Ok(resp.json().await?)
    } else {
        Err(resp.json::<BackblazeResponseError>().await?.into())
    }
}
