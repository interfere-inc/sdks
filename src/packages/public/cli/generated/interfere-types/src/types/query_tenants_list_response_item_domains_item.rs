pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTenantsListResponseItemDomainsItem {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
    #[serde(rename = "verifiedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub verified_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
}

impl QueryTenantsListResponseItemDomainsItem {
    pub fn builder() -> QueryTenantsListResponseItemDomainsItemBuilder {
        <QueryTenantsListResponseItemDomainsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsListResponseItemDomainsItemBuilder {
    domain: Option<String>,
    id: Option<String>,
    kind: Option<String>,
    organization_id: Option<String>,
    source: Option<String>,
    tenant_id: Option<String>,
    verified_at: Option<f64>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryTenantsListResponseItemDomainsItemBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn verified_at(mut self, value: f64) -> Self {
        self.verified_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsListResponseItemDomainsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](QueryTenantsListResponseItemDomainsItemBuilder::domain)
    /// - [`id`](QueryTenantsListResponseItemDomainsItemBuilder::id)
    /// - [`kind`](QueryTenantsListResponseItemDomainsItemBuilder::kind)
    /// - [`organization_id`](QueryTenantsListResponseItemDomainsItemBuilder::organization_id)
    /// - [`source`](QueryTenantsListResponseItemDomainsItemBuilder::source)
    /// - [`tenant_id`](QueryTenantsListResponseItemDomainsItemBuilder::tenant_id)
    /// - [`created_at`](QueryTenantsListResponseItemDomainsItemBuilder::created_at)
    /// - [`updated_at`](QueryTenantsListResponseItemDomainsItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryTenantsListResponseItemDomainsItem, BuildError> {
        Ok(QueryTenantsListResponseItemDomainsItem {
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            verified_at: self.verified_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
