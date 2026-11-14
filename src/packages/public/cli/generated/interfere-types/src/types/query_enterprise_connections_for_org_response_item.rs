pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryEnterpriseConnectionsForOrgResponseItem {
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "clerkEnterpriseConnectionId")]
    #[serde(default)]
    pub clerk_enterprise_connection_id: String,
    #[serde(rename = "clerkUpdatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub clerk_updated_at: Option<f64>,
    #[serde(rename = "disableAdditionalIdentifications")]
    #[serde(default)]
    pub disable_additional_identifications: bool,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "logoPublicUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_public_url: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(default)]
    pub provider: String,
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

impl QueryEnterpriseConnectionsForOrgResponseItem {
    pub fn builder() -> QueryEnterpriseConnectionsForOrgResponseItemBuilder {
        <QueryEnterpriseConnectionsForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryEnterpriseConnectionsForOrgResponseItemBuilder {
    active: Option<bool>,
    clerk_enterprise_connection_id: Option<String>,
    clerk_updated_at: Option<f64>,
    disable_additional_identifications: Option<bool>,
    domains: Option<Vec<String>>,
    id: Option<String>,
    logo_public_url: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    protocol: Option<String>,
    provider: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryEnterpriseConnectionsForOrgResponseItemBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn clerk_enterprise_connection_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_enterprise_connection_id = Some(value.into());
        self
    }

    pub fn clerk_updated_at(mut self, value: f64) -> Self {
        self.clerk_updated_at = Some(value);
        self
    }

    pub fn disable_additional_identifications(mut self, value: bool) -> Self {
        self.disable_additional_identifications = Some(value);
        self
    }

    pub fn domains(mut self, value: Vec<String>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_public_url(mut self, value: impl Into<String>) -> Self {
        self.logo_public_url = Some(value.into());
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

    pub fn protocol(mut self, value: impl Into<String>) -> Self {
        self.protocol = Some(value.into());
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryEnterpriseConnectionsForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::active)
    /// - [`clerk_enterprise_connection_id`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::clerk_enterprise_connection_id)
    /// - [`disable_additional_identifications`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::disable_additional_identifications)
    /// - [`domains`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::domains)
    /// - [`id`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::id)
    /// - [`name`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::name)
    /// - [`organization_id`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::organization_id)
    /// - [`provider`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::provider)
    /// - [`created_at`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryEnterpriseConnectionsForOrgResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryEnterpriseConnectionsForOrgResponseItem, BuildError> {
        Ok(QueryEnterpriseConnectionsForOrgResponseItem {
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            clerk_enterprise_connection_id: self.clerk_enterprise_connection_id.ok_or_else(|| BuildError::missing_field("clerk_enterprise_connection_id"))?,
            clerk_updated_at: self.clerk_updated_at,
            disable_additional_identifications: self.disable_additional_identifications.ok_or_else(|| BuildError::missing_field("disable_additional_identifications"))?,
            domains: self.domains.ok_or_else(|| BuildError::missing_field("domains"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_public_url: self.logo_public_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            protocol: self.protocol,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
