use reqwest::Client;

use crate::{
    AccountAuthorization, BackblazeResponseError, Result, encode_file_name,
    endpoint::ListFileVersionsResponse,
};

/// https://www.backblaze.com/b2/docs/b2_list_file_versions.html
pub async fn list_file_versions(
    bucket_id: &str,
    start_file_name: Option<&str>,
    start_file_id: Option<&str>,
    max_file_count: Option<usize>,
    prefix: Option<&str>,
    delimiter: Option<&str>,
    auth: &AccountAuthorization,
    client: &Client,
) -> Result<ListFileVersionsResponse> {
    let mut query = vec![format!("bucketId={bucket_id}")];

    if let Some(value) = start_file_name {
        query.push(format!(
            "startFileName={}",
            urlencoding::encode(&encode_file_name(value))
        ));
    }

    if let Some(value) = start_file_id {
        query.push(format!("startFileId={value}"));
    }

    if let Some(value) = max_file_count {
        query.push(format!("maxFileCount={value}"));
    }

    if let Some(value) = prefix {
        query.push(format!("prefix={}", urlencoding::encode(value)));
    }

    if let Some(value) = delimiter {
        query.push(format!("delimiter={}", urlencoding::encode(value)));
    }

    println!("{query:#?}");

    let resp = client
        .get(
            format!(
                "{}/b2api/v4/b2_list_file_versions?{}",
                auth.api_info.storage_api.api_url,
                query.join("&")
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
