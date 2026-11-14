pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryDomainsForOrgResponseItemDomainMetadata {
    #[serde(rename = "cfHostnameId")]
    #[serde(default)]
    pub cf_hostname_id: String,
    /// Current TLS certificate provisioning status.
    #[serde(rename = "cfSslStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cf_ssl_status: Option<String>,
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<QueryDomainsForOrgResponseItemDomainMetadataProvider>,
}

impl QueryDomainsForOrgResponseItemDomainMetadata {
    pub fn builder() -> QueryDomainsForOrgResponseItemDomainMetadataBuilder {
        <QueryDomainsForOrgResponseItemDomainMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryDomainsForOrgResponseItemDomainMetadataBuilder {
    cf_hostname_id: Option<String>,
    cf_ssl_status: Option<String>,
    last_error: Option<String>,
    provider: Option<QueryDomainsForOrgResponseItemDomainMetadataProvider>,
}

impl QueryDomainsForOrgResponseItemDomainMetadataBuilder {
    pub fn cf_hostname_id(mut self, value: impl Into<String>) -> Self {
        self.cf_hostname_id = Some(value.into());
        self
    }

    pub fn cf_ssl_status(mut self, value: impl Into<String>) -> Self {
        self.cf_ssl_status = Some(value.into());
        self
    }

    pub fn last_error(mut self, value: impl Into<String>) -> Self {
        self.last_error = Some(value.into());
        self
    }

    pub fn provider(mut self, value: QueryDomainsForOrgResponseItemDomainMetadataProvider) -> Self {
        self.provider = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryDomainsForOrgResponseItemDomainMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cf_hostname_id`](QueryDomainsForOrgResponseItemDomainMetadataBuilder::cf_hostname_id)
    pub fn build(self) -> Result<QueryDomainsForOrgResponseItemDomainMetadata, BuildError> {
        Ok(QueryDomainsForOrgResponseItemDomainMetadata {
            cf_hostname_id: self.cf_hostname_id.ok_or_else(|| BuildError::missing_field("cf_hostname_id"))?,
            cf_ssl_status: self.cf_ssl_status,
            last_error: self.last_error,
            provider: self.provider,
        })
    }
}
