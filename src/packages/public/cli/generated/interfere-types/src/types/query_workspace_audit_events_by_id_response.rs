pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryWorkspaceAuditEventsByIdResponse {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(default)]
    pub category: String,
    #[serde(rename = "actorName")]
    #[serde(default)]
    pub actor_name: String,
    #[serde(rename = "actorEmail")]
    #[serde(default)]
    pub actor_email: String,
    #[serde(default)]
    pub action: String,
    #[serde(rename = "targetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
    #[serde(rename = "targetLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_label: Option<String>,
    #[serde(default)]
    pub metadata: HashMap<String, Option<QueryWorkspaceAuditEventsByIdResponseMetadataValue>>,
    #[serde(default)]
    pub source: String,
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "actorUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_user_id: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<QueryWorkspaceAuditEventsByIdResponseActor>,
}

impl QueryWorkspaceAuditEventsByIdResponse {
    pub fn builder() -> QueryWorkspaceAuditEventsByIdResponseBuilder {
        <QueryWorkspaceAuditEventsByIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryWorkspaceAuditEventsByIdResponseBuilder {
    created_at: Option<f64>,
    category: Option<String>,
    actor_name: Option<String>,
    actor_email: Option<String>,
    action: Option<String>,
    target_id: Option<String>,
    target_label: Option<String>,
    metadata: Option<HashMap<String, Option<QueryWorkspaceAuditEventsByIdResponseMetadataValue>>>,
    source: Option<String>,
    request_id: Option<String>,
    actor_user_id: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    actor: Option<QueryWorkspaceAuditEventsByIdResponseActor>,
}

impl QueryWorkspaceAuditEventsByIdResponseBuilder {
    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn actor_name(mut self, value: impl Into<String>) -> Self {
        self.actor_name = Some(value.into());
        self
    }

    pub fn actor_email(mut self, value: impl Into<String>) -> Self {
        self.actor_email = Some(value.into());
        self
    }

    pub fn action(mut self, value: impl Into<String>) -> Self {
        self.action = Some(value.into());
        self
    }

    pub fn target_id(mut self, value: impl Into<String>) -> Self {
        self.target_id = Some(value.into());
        self
    }

    pub fn target_label(mut self, value: impl Into<String>) -> Self {
        self.target_label = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<QueryWorkspaceAuditEventsByIdResponseMetadataValue>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn actor_user_id(mut self, value: impl Into<String>) -> Self {
        self.actor_user_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn actor(mut self, value: QueryWorkspaceAuditEventsByIdResponseActor) -> Self {
        self.actor = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryWorkspaceAuditEventsByIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](QueryWorkspaceAuditEventsByIdResponseBuilder::created_at)
    /// - [`category`](QueryWorkspaceAuditEventsByIdResponseBuilder::category)
    /// - [`actor_name`](QueryWorkspaceAuditEventsByIdResponseBuilder::actor_name)
    /// - [`actor_email`](QueryWorkspaceAuditEventsByIdResponseBuilder::actor_email)
    /// - [`action`](QueryWorkspaceAuditEventsByIdResponseBuilder::action)
    /// - [`metadata`](QueryWorkspaceAuditEventsByIdResponseBuilder::metadata)
    /// - [`source`](QueryWorkspaceAuditEventsByIdResponseBuilder::source)
    /// - [`id`](QueryWorkspaceAuditEventsByIdResponseBuilder::id)
    /// - [`organization_id`](QueryWorkspaceAuditEventsByIdResponseBuilder::organization_id)
    pub fn build(self) -> Result<QueryWorkspaceAuditEventsByIdResponse, BuildError> {
        Ok(QueryWorkspaceAuditEventsByIdResponse {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            category: self.category.ok_or_else(|| BuildError::missing_field("category"))?,
            actor_name: self.actor_name.ok_or_else(|| BuildError::missing_field("actor_name"))?,
            actor_email: self.actor_email.ok_or_else(|| BuildError::missing_field("actor_email"))?,
            action: self.action.ok_or_else(|| BuildError::missing_field("action"))?,
            target_id: self.target_id,
            target_label: self.target_label,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            request_id: self.request_id,
            actor_user_id: self.actor_user_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            actor: self.actor,
        })
    }
}
