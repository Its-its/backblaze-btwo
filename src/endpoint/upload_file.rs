use {
    super::{
        super::{BackblazeResponseError, Result, encode_file_name},
        UploadFileResponse, UploadUrlResponse,
    },
    base16ct::upper::encode_string,
    reqwest::Client,
    sha1::{Digest, Sha1},
};

/// https://www.backblaze.com/b2/docs/b2_upload_file.html
pub async fn upload_file(
    file_name: &str,
    content_type: &str,
    contents: Vec<u8>,
    upload: &UploadUrlResponse,
    client: &Client,
) -> Result<UploadFileResponse> {
    let sha = encode_string(&Sha1::digest(contents.as_slice()));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha1() {
        let sha = encode_string(&Sha1::digest(b"abc"));
        assert_eq!("A9993E364706816ABA3E25717850C26C9CD0D89D", sha);
    }
}
