use reqwest::{Client, Response};

use crate::{encode_file_name, AccountAuthorization, BackblazeResponseError, Result};

/// https://www.backblaze.com/b2/docs/b2_download_file_by_name.html
pub async fn download_file_by_name(
    bucket_name: &str,
    file_path: &str,
    auth: AccountAuthorization,
    client: &Client,
) -> Result<Response> {
    let resp = client
        .get(
            format!(
                "{}/file/{bucket_name}/{}",
                auth.download_url,
                encode_file_name(file_path)
            )
            .as_str(),
        )
        .header("Authorization", auth.authorization_token.as_str())
        .send()
        .await?;

    if resp.status().as_u16() == 200 {
        Ok(resp)
    } else {
        Err(resp.json::<BackblazeResponseError>().await?.into())
    }
}
