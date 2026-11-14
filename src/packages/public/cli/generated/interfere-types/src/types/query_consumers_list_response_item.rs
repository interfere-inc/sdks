pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryConsumersListResponseItem {
    #[serde(rename = "anonymousNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "endUserOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_org_id: Option<String>,
    #[serde(rename = "erroredAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub errored_at: Option<f64>,
    #[serde(rename = "firstSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub first_seen_at: Option<f64>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    #[serde(rename = "lastSeenAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_seen_at: Option<f64>,
    #[serde(rename = "mergedIntoId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_into_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    /// Auth provider that surfaced this identification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<QueryConsumersListResponseItemSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traits: Option<QueryConsumersListResponseItemTraits>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<QueryConsumersListResponseItemTenant>,
}

impl QueryConsumersListResponseItem {
    pub fn builder() -> QueryConsumersListResponseItemBuilder {
        <QueryConsumersListResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersListResponseItemBuilder {
    anonymous_number: Option<i64>,
    avatar: Option<String>,
    country: Option<String>,
    email: Option<String>,
    end_user_org_id: Option<String>,
    errored_at: Option<f64>,
    first_seen_at: Option<f64>,
    id: Option<String>,
    identifier: Option<String>,
    last_seen_at: Option<f64>,
    merged_into_id: Option<String>,
    name: Option<String>,
    organization_id: Option<String>,
    source: Option<QueryConsumersListResponseItemSource>,
    traits: Option<QueryConsumersListResponseItemTraits>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    tenant: Option<QueryConsumersListResponseItemTenant>,
}

impl QueryConsumersListResponseItemBuilder {
    pub fn anonymous_number(mut self, value: i64) -> Self {
        self.anonymous_number = Some(value);
        self
    }

    pub fn avatar(mut self, value: impl Into<String>) -> Self {
        self.avatar = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn end_user_org_id(mut self, value: impl Into<String>) -> Self {
        self.end_user_org_id = Some(value.into());
        self
    }

    pub fn errored_at(mut self, value: f64) -> Self {
        self.errored_at = Some(value);
        self
    }

    pub fn first_seen_at(mut self, value: f64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
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

    pub fn source(mut self, value: QueryConsumersListResponseItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn traits(mut self, value: QueryConsumersListResponseItemTraits) -> Self {
        self.traits = Some(value);
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

    pub fn tenant(mut self, value: QueryConsumersListResponseItemTenant) -> Self {
        self.tenant = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersListResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryConsumersListResponseItemBuilder::id)
    /// - [`organization_id`](QueryConsumersListResponseItemBuilder::organization_id)
    /// - [`created_at`](QueryConsumersListResponseItemBuilder::created_at)
    /// - [`updated_at`](QueryConsumersListResponseItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryConsumersListResponseItem, BuildError> {
        Ok(QueryConsumersListResponseItem {
            anonymous_number: self.anonymous_number,
            avatar: self.avatar,
            country: self.country,
            email: self.email,
            end_user_org_id: self.end_user_org_id,
            errored_at: self.errored_at,
            first_seen_at: self.first_seen_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            identifier: self.identifier,
            last_seen_at: self.last_seen_at,
            merged_into_id: self.merged_into_id,
            name: self.name,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            source: self.source,
            traits: self.traits,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            tenant: self.tenant,
        })
    }
}
