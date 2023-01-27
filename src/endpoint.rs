use serde::Deserialize;
use serde_json::Value;

mod get_upload_url;
mod upload_file;
mod hide_file;
mod start_large_file;
mod get_upload_part_url;
mod upload_part;
mod finish_large_file;
mod cancel_large_file;
mod download_file_by_name;
mod authorize_account;

pub use get_upload_url::*;
pub use upload_file::*;
pub use hide_file::*;
pub use start_large_file::*;
pub use get_upload_part_url::*;
pub use upload_part::*;
pub use finish_large_file::*;
pub use cancel_large_file::*;
pub use download_file_by_name::*;
pub use authorize_account::*;

use crate::{BucketId, ApplicationKeyId, FileId};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Action {
    /// start means that a large file has been started, but not finished or canceled.
    Start,
    /// upload means a file that was uploaded to B2 Cloud Storage.
    Upload,
    /// hide means a file version marking the file as hidden, so that it will not show up in b2_list_file_names.
    Hide,
    /// folder is used to indicate a virtual folder when listing files.
    Folder,
    /// Untagged action.
    Unknown(String),
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadFileResponse {
    pub account_id: ApplicationKeyId,
    pub action: Action,
    pub bucket_id: BucketId,
    pub content_length: usize,
    pub content_md5: Option<String>,
    pub content_sha1: String,
    pub content_type: String,
    pub file_id: FileId,
    /// This is a JSON object, holding the name/value pairs that were uploaded with the file.
    pub file_info: Value,
    pub file_name: String,
    pub file_retention: Option<FileRetention>,
    pub legal_hold: Option<LegalHold>,
    pub replication_status: Option<ReplicationStatus>,
    pub server_side_encryption: Option<EncryptionConfig>,
    pub upload_timestamp: usize,
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum ReplicationStatus {
    Pending,
    Completed,
    Failed,
    Replica
}


#[derive(Debug)]
pub enum LegalHold {
    On,
    Off,
}

impl<'de> Deserialize<'de> for LegalHold {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        if String::deserialize(deserializer)? == "on" {
            Ok(Self::On)
        } else {
            Ok(Self::Off)
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionConfig {
    pub mode: Option<String>,
    pub algorithm: Option<String>,
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRetention {
    pub is_client_authorized_to_read: bool,

    pub value: Option<FileRetentionValue>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRetentionValue {
    pub mode: FileRetentionMode,
    pub retain_until_timestamp: usize,
}


#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum FileRetentionMode {
    Compliance,
    Governance,

    Unknown(String),
}