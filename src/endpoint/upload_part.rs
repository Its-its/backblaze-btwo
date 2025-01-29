use std::num::NonZeroUsize;

use bytes::Bytes;
use reqwest::Client;
use serde::Deserialize;
use sha1::{Digest, Sha1};

use super::{EncryptionConfig, UploadPartUrlResponse};
use crate::{BackblazeResponseError, Result};

/// https://www.backblaze.com/b2/docs/b2_upload_part.html
pub async fn upload_part(
    part_number: NonZeroUsize,
    contents: Bytes,
    upload: &UploadPartUrlResponse,
    client: &Client,
) -> Result<UploadPartResponse> {
    let sha = format!("{:X}", Sha1::digest(&contents));

    let resp = client
        .post(upload.upload_url.as_str())
        .header("Authorization", upload.authorization_token.as_str())
        .header("Content-Length", contents.len())
        .header("X-Bz-Part-Number", part_number.get())
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadPartResponse {
    pub file_id: String,
    pub part_number: usize,
    pub content_length: usize,
    pub content_md5: String,
    pub server_side_encryption: Option<EncryptionConfig>,
    pub upload_timestamp: usize,
}
