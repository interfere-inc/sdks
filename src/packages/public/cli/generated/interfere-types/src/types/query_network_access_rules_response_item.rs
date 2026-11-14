pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryNetworkAccessRulesResponseItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn: Option<i64>,
    #[serde(rename = "asnGeneration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_generation: Option<i64>,
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
    #[serde(rename = "networkAsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_asn: Option<QueryNetworkAccessRulesResponseItemNetworkAsn>,
}

impl QueryNetworkAccessRulesResponseItem {
    pub fn builder() -> QueryNetworkAccessRulesResponseItemBuilder {
        <QueryNetworkAccessRulesResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryNetworkAccessRulesResponseItemBuilder {
    id: Option<String>,
    organization_id: Option<String>,
    label: Option<String>,
    cidr: Option<String>,
    asn: Option<i64>,
    asn_generation: Option<i64>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    network_asn: Option<QueryNetworkAccessRulesResponseItemNetworkAsn>,
}

impl QueryNetworkAccessRulesResponseItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn cidr(mut self, value: impl Into<String>) -> Self {
        self.cidr = Some(value.into());
        self
    }

    pub fn asn(mut self, value: i64) -> Self {
        self.asn = Some(value);
        self
    }

    pub fn asn_generation(mut self, value: i64) -> Self {
        self.asn_generation = Some(value);
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

    pub fn network_asn(mut self, value: QueryNetworkAccessRulesResponseItemNetworkAsn) -> Self {
        self.network_asn = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryNetworkAccessRulesResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryNetworkAccessRulesResponseItemBuilder::id)
    /// - [`organization_id`](QueryNetworkAccessRulesResponseItemBuilder::organization_id)
    /// - [`label`](QueryNetworkAccessRulesResponseItemBuilder::label)
    /// - [`created_at`](QueryNetworkAccessRulesResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryNetworkAccessRulesResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryNetworkAccessRulesResponseItem, BuildError> {
        Ok(QueryNetworkAccessRulesResponseItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            cidr: self.cidr,
            asn: self.asn,
            asn_generation: self.asn_generation,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            network_asn: self.network_asn,
        })
    }
}
