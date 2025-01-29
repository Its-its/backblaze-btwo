use reqwest::Client;
use sha1::{Digest, Sha1};

use crate::{encode_file_name, BackblazeResponseError, Result};

use super::{UploadFileResponse, UploadUrlResponse};

/// https://www.backblaze.com/b2/docs/b2_upload_file.html
pub async fn upload_file(
    file_name: &str,
    content_type: &str,
    contents: Vec<u8>,
    upload: &UploadUrlResponse,
    client: &Client,
) -> Result<UploadFileResponse> {
    let sha = format!("{:X}", Sha1::digest(contents.as_slice()));

    let resp = client
        .post(upload.upload_url.as_str())
        .header("Authorization", upload.authorization_token.as_str())
        .header("Content-Type", content_type)
        .header("Content-Length", contents.len())
        .header("X-Bz-File-Name", encode_file_name(file_name).as_str())
        .header("X-Bz-Content-Sha1", sha.as_str())
        .body(contents)
        .send()
        .await?;

    if resp.status().as_u16() == 200 {
        Ok(resp.json().await?)
    } else {
        Err(resp.json::<BackblazeResponseError>().await?.into())
    }
}
