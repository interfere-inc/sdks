pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QuerySupportThreadsForOrgResponseItem {
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
    #[serde(rename = "createdByUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastMessageAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_message_at: Option<f64>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "previewText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_text: Option<String>,
    pub status: QuerySupportThreadsForOrgResponseItemStatus,
    #[serde(default)]
    pub title: String,
}

impl QuerySupportThreadsForOrgResponseItem {
    pub fn builder() -> QuerySupportThreadsForOrgResponseItemBuilder {
        <QuerySupportThreadsForOrgResponseItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QuerySupportThreadsForOrgResponseItemBuilder {
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    created_by_user_id: Option<String>,
    id: Option<String>,
    last_message_at: Option<f64>,
    organization_id: Option<String>,
    preview_text: Option<String>,
    status: Option<QuerySupportThreadsForOrgResponseItemStatus>,
    title: Option<String>,
}

impl QuerySupportThreadsForOrgResponseItemBuilder {
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

    pub fn created_by_user_id(mut self, value: impl Into<String>) -> Self {
        self.created_by_user_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_message_at(mut self, value: f64) -> Self {
        self.last_message_at = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn preview_text(mut self, value: impl Into<String>) -> Self {
        self.preview_text = Some(value.into());
        self
    }

    pub fn status(mut self, value: QuerySupportThreadsForOrgResponseItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QuerySupportThreadsForOrgResponseItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QuerySupportThreadsForOrgResponseItemBuilder::created_at)
    /// - [`updated_at`](QuerySupportThreadsForOrgResponseItemBuilder::updated_at)
    /// - [`id`](QuerySupportThreadsForOrgResponseItemBuilder::id)
    /// - [`organization_id`](QuerySupportThreadsForOrgResponseItemBuilder::organization_id)
    /// - [`status`](QuerySupportThreadsForOrgResponseItemBuilder::status)
    /// - [`title`](QuerySupportThreadsForOrgResponseItemBuilder::title)
    pub fn build(self) -> Result<QuerySupportThreadsForOrgResponseItem, BuildError> {
        Ok(QuerySupportThreadsForOrgResponseItem {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_by_user_id: self.created_by_user_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_message_at: self.last_message_at,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            preview_text: self.preview_text,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title.ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
