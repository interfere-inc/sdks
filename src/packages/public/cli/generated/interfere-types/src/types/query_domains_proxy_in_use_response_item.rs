pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryDomainsProxyInUseResponseItem {
    #[serde(rename = "affiliationEmailAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affiliation_email_address: Option<String>,
    #[serde(rename = "clerkDomainId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clerk_domain_id: Option<String>,
    #[serde(rename = "domainMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_metadata: Option<QueryDomainsProxyInUseResponseItemDomainMetadata>,
    #[serde(rename = "enrollmentMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrollment_mode: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<QueryDomainsProxyInUseResponseItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<QueryDomainsProxyInUseResponseItemType>,
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

impl QueryDomainsProxyInUseResponseItem {
    pub fn builder() -> QueryDomainsProxyInUseResponseItemBuilder {
        <QueryDomainsProxyInUseResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryDomainsProxyInUseResponseItemBuilder {
    affiliation_email_address: Option<String>,
    clerk_domain_id: Option<String>,
    domain_metadata: Option<QueryDomainsProxyInUseResponseItemDomainMetadata>,
    enrollment_mode: Option<String>,
    id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    status: Option<QueryDomainsProxyInUseResponseItemStatus>,
    r#type: Option<QueryDomainsProxyInUseResponseItemType>,
    verified_at: Option<f64>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryDomainsProxyInUseResponseItemBuilder {
    pub fn affiliation_email_address(mut self, value: impl Into<String>) -> Self {
        self.affiliation_email_address = Some(value.into());
        self
    }

    pub fn clerk_domain_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_domain_id = Some(value.into());
        self
    }

    pub fn domain_metadata(mut self, value: QueryDomainsProxyInUseResponseItemDomainMetadata) -> Self {
        self.domain_metadata = Some(value);
        self
    }

    pub fn enrollment_mode(mut self, value: impl Into<String>) -> Self {
        self.enrollment_mode = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn status(mut self, value: QueryDomainsProxyInUseResponseItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryDomainsProxyInUseResponseItemType) -> Self {
        self.r#type = Some(value);
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

    /// Consumes the builder and constructs a [`QueryDomainsProxyInUseResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryDomainsProxyInUseResponseItemBuilder::id)
    /// - [`name`](QueryDomainsProxyInUseResponseItemBuilder::name)
    /// - [`organization_id`](QueryDomainsProxyInUseResponseItemBuilder::organization_id)
    /// - [`created_at`](QueryDomainsProxyInUseResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryDomainsProxyInUseResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryDomainsProxyInUseResponseItem, BuildError> {
        Ok(QueryDomainsProxyInUseResponseItem {
            affiliation_email_address: self.affiliation_email_address,
            clerk_domain_id: self.clerk_domain_id,
            domain_metadata: self.domain_metadata,
            enrollment_mode: self.enrollment_mode,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            status: self.status,
            r#type: self.r#type,
            verified_at: self.verified_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
