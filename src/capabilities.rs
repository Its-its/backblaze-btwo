use serde::{Serialize, Deserialize};



#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    ListKeys,
    WriteKeys,
    DeleteKeys,
    ListBuckets,
    WriteBuckets,
    DeleteBuckets,
    ListFiles,
    ReadFiles,
    ShareFiles,
    WriteFiles,
    DeleteFiles,
}