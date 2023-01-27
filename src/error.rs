use serde::{Serialize, Deserialize};


pub type Result<V, E = Error> = std::result::Result<V, E>;


#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request Error {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("JSON Error {0}")]
    Json(#[from] serde_json::Error),

    #[error("Backblaze B2 Response Error {0}")]
    Backblaze(#[from] BackblazeResponseError),

    #[error("Backblaze B2 Authorization Error.")]
    AccountAuthorization,
    #[error("Backblaze B2 Get Upload Url Error.")]
    B2GetUploadUrl,
    #[error("Backblaze B2 Upload File Error.")]
    B2UploadFile,
}


#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
#[error("Backblaze Error:\nStatus: {status},\nCode: {code:?},\nMessage: {message}")]
pub struct BackblazeResponseError {
    status: isize,
    code: BackblazeErrorIdentifier,
    message: String,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum BackblazeErrorIdentifier {
    // 400
    BadBucketId,
    CannotDeleteNonEmptyBucket,
    SourceTooLarge,
    TooManyBuckets,
    DuplicateBucketName,
    FileNotPresent,
    OutOfRange,
    InvalidBucketId,
    CustomTimestampInvalid,
    CustomTimestampNotAllowed,
    FileLockConflict,
    RestrictedBucketConflict,
    SourceReplicationConflict,
    MetadataExceeded,
    AuthTokenLimit,

    // 401
    Unauthorized,
    Unsupported,
    BadAuthToken,
    ExpiredAuthToken,

    // 403
    DownloadCapExceeded,
    TransactionCapExceeded,
    StorageCapExceeded,
    CapExceeded,

    // 401/403
    AccessDenied,

    // 404
    NotFound,

    // 405
    MethodNotAllowed,

    // 408
    RequestTimeout,

    // 409
    Conflict,

    // 416
    RangeNotSatisfiable,

    // 429
    TooManyRequests,

    // 503
    ServiceUnavailable,

    // 400/503
    BadRequest,

    // Unknown
    Unknown(String),
}