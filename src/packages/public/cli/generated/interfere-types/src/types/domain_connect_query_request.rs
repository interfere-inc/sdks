pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for domainConnect
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainConnectQueryRequest {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "redirectUri")]
    #[serde(default)]
    pub redirect_uri: String,
}

impl DomainConnectQueryRequest {
    pub fn builder() -> DomainConnectQueryRequestBuilder {
        <DomainConnectQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainConnectQueryRequestBuilder {
    name: Option<String>,
    redirect_uri: Option<String>,
}

impl DomainConnectQueryRequestBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn redirect_uri(mut self, value: impl Into<String>) -> Self {
        self.redirect_uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainConnectQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](DomainConnectQueryRequestBuilder::name)
    /// - [`redirect_uri`](DomainConnectQueryRequestBuilder::redirect_uri)
    pub fn build(self) -> Result<DomainConnectQueryRequest, BuildError> {
        Ok(DomainConnectQueryRequest {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            redirect_uri: self.redirect_uri.ok_or_else(|| BuildError::missing_field("redirect_uri"))?,
        })
    }
}

