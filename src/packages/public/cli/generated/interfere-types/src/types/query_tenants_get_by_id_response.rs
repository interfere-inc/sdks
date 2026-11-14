pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTenantsGetByIdResponse {
    #[serde(default)]
    pub domain: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_seen_at: Option<f64>,
    #[serde(rename = "logoUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    #[serde(rename = "mergedIntoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
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
    #[serde(default)]
    pub domains: Vec<QueryTenantsGetByIdResponseDomainsItem>,
}

impl QueryTenantsGetByIdResponse {
    pub fn builder() -> QueryTenantsGetByIdResponseBuilder {
        <QueryTenantsGetByIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsGetByIdResponseBuilder {
    domain: Option<String>,
    id: Option<String>,
    last_seen_at: Option<f64>,
    logo_url: Option<String>,
    merged_into_id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    tier: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    domains: Option<Vec<QueryTenantsGetByIdResponseDomainsItem>>,
}

impl QueryTenantsGetByIdResponseBuilder {
    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn merged_into_id(mut self, value: impl Into<String>) -> Self {
        self.merged_into_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn tier(mut self, value: impl Into<String>) -> Self {
        self.tier = Some(value.into());
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

    pub fn domains(mut self, value: Vec<QueryTenantsGetByIdResponseDomainsItem>) -> Self {
        self.domains = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsGetByIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`domain`](QueryTenantsGetByIdResponseBuilder::domain)
    /// - [`id`](QueryTenantsGetByIdResponseBuilder::id)
    /// - [`organization_id`](QueryTenantsGetByIdResponseBuilder::organization_id)
    /// - [`created_at`](QueryTenantsGetByIdResponseBuilder::created_at)
    /// - [`updated_at`](QueryTenantsGetByIdResponseBuilder::updated_at)
    /// - [`domains`](QueryTenantsGetByIdResponseBuilder::domains)
    pub fn build(self) -> Result<QueryTenantsGetByIdResponse, BuildError> {
        Ok(QueryTenantsGetByIdResponse {
            domain: self.domain.ok_or_else(|| BuildError::missing_field("domain"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_seen_at: self.last_seen_at,
            logo_url: self.logo_url,
            merged_into_id: self.merged_into_id,
            name: self.name,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            tier: self.tier,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            domains: self.domains.ok_or_else(|| BuildError::missing_field("domains"))?,
        })
    }
}
