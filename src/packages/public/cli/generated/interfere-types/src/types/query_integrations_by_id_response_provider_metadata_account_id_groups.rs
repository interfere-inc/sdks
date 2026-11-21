pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryIntegrationsByIdResponseProviderMetadataAccountIdGroups {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsStatus>,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(rename = "syncedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub synced_at: Option<i64>,
}

impl QueryIntegrationsByIdResponseProviderMetadataAccountIdGroups {
    pub fn builder() -> QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsBuilder {
        <QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsBuilder {
    account: Option<String>,
    enabled: Option<bool>,
    status: Option<QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsStatus>,
    synced_at: Option<i64>,
}

impl QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsBuilder {
    pub fn account(mut self, value: impl Into<String>) -> Self {
        self.account = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn status(mut self, value: QueryIntegrationsByIdResponseProviderMetadataAccountIdGroupsStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn synced_at(mut self, value: i64) -> Self {
        self.synced_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsByIdResponseProviderMetadataAccountIdGroups`].
    pub fn build(self) -> Result<QueryIntegrationsByIdResponseProviderMetadataAccountIdGroups, BuildError> {
        Ok(QueryIntegrationsByIdResponseProviderMetadataAccountIdGroups {
            account: self.account,
            enabled: self.enabled,
            status: self.status,
            synced_at: self.synced_at,
        })
    }
}
