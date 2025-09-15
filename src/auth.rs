use base64::{engine::general_purpose, Engine as _};
use reqwest::Client;

use crate::{
    endpoint::{self, AccountAuthorization},
    ApplicationKey, ApplicationKeyId, Result,
};

#[derive(Clone)]
pub struct Credentials {
    pub id: ApplicationKeyId,
    pub key: ApplicationKey,
}

impl Credentials {
    pub fn new(id: impl Into<ApplicationKeyId>, key: impl Into<ApplicationKey>) -> Self {
        Self {
            id: id.into(),
            key: key.into(),
        }
    }

    pub const fn new_const(id: ApplicationKeyId, key: ApplicationKey) -> Self {
        Self { id, key }
    }

    pub(crate) fn header_name(&self) -> &str {
        "Authorization"
    }

    pub(crate) fn id_key(&self) -> String {
        format!("{}:{}", self.id, self.key)
    }

    pub(crate) fn auth_string(&self) -> String {
        format!("Basic {}", general_purpose::STANDARD.encode(self.id_key()))
    }

    pub async fn authorize(&self, client: &Client) -> Result<AccountAuthorization> {
        endpoint::authorize_account(self, client).await
    }
}
