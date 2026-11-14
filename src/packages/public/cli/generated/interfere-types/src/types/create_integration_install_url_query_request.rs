pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for createIntegrationInstallUrl
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateIntegrationInstallUrlQueryRequest {
    #[serde(rename = "returnPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl CreateIntegrationInstallUrlQueryRequest {
    pub fn builder() -> CreateIntegrationInstallUrlQueryRequestBuilder {
        <CreateIntegrationInstallUrlQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateIntegrationInstallUrlQueryRequestBuilder {
    return_path: Option<String>,
    host: Option<String>,
    account_id: Option<String>,
}

impl CreateIntegrationInstallUrlQueryRequestBuilder {
    pub fn return_path(mut self, value: impl Into<String>) -> Self {
        self.return_path = Some(value.into());
        self
    }

    pub fn host(mut self, value: impl Into<String>) -> Self {
        self.host = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateIntegrationInstallUrlQueryRequest`].
    pub fn build(self) -> Result<CreateIntegrationInstallUrlQueryRequest, BuildError> {
        Ok(CreateIntegrationInstallUrlQueryRequest {
            return_path: self.return_path,
            host: self.host,
            account_id: self.account_id,
        })
    }
}

