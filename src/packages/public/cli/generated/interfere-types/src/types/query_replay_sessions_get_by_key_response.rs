pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryReplaySessionsGetByKeyResponse {
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "firstEventTs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub first_event_ts: f64,
    #[serde(rename = "hasFullSnapshot")]
    #[serde(default)]
    pub has_full_snapshot: bool,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "lastEventTs")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_event_ts: f64,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub started_at: f64,
    #[serde(rename = "storageDeletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub storage_deleted_at: Option<f64>,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
}

impl QueryReplaySessionsGetByKeyResponse {
    pub fn builder() -> QueryReplaySessionsGetByKeyResponseBuilder {
        <QueryReplaySessionsGetByKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryReplaySessionsGetByKeyResponseBuilder {
    deleted_at: Option<f64>,
    first_event_ts: Option<f64>,
    has_full_snapshot: Option<bool>,
    id: Option<String>,
    last_event_ts: Option<f64>,
    organization_id: Option<String>,
    session_id: Option<String>,
    started_at: Option<f64>,
    storage_deleted_at: Option<f64>,
    surface_id: Option<String>,
    updated_at: Option<f64>,
}

impl QueryReplaySessionsGetByKeyResponseBuilder {
    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn first_event_ts(mut self, value: f64) -> Self {
        self.first_event_ts = Some(value);
        self
    }

    pub fn has_full_snapshot(mut self, value: bool) -> Self {
        self.has_full_snapshot = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_event_ts(mut self, value: f64) -> Self {
        self.last_event_ts = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn started_at(mut self, value: f64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn storage_deleted_at(mut self, value: f64) -> Self {
        self.storage_deleted_at = Some(value);
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryReplaySessionsGetByKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_event_ts`](QueryReplaySessionsGetByKeyResponseBuilder::first_event_ts)
    /// - [`has_full_snapshot`](QueryReplaySessionsGetByKeyResponseBuilder::has_full_snapshot)
    /// - [`id`](QueryReplaySessionsGetByKeyResponseBuilder::id)
    /// - [`last_event_ts`](QueryReplaySessionsGetByKeyResponseBuilder::last_event_ts)
    /// - [`organization_id`](QueryReplaySessionsGetByKeyResponseBuilder::organization_id)
    /// - [`session_id`](QueryReplaySessionsGetByKeyResponseBuilder::session_id)
    /// - [`started_at`](QueryReplaySessionsGetByKeyResponseBuilder::started_at)
    /// - [`updated_at`](QueryReplaySessionsGetByKeyResponseBuilder::updated_at)
    pub fn build(self) -> Result<QueryReplaySessionsGetByKeyResponse, BuildError> {
        Ok(QueryReplaySessionsGetByKeyResponse {
            deleted_at: self.deleted_at,
            first_event_ts: self.first_event_ts.ok_or_else(|| BuildError::missing_field("first_event_ts"))?,
            has_full_snapshot: self.has_full_snapshot.ok_or_else(|| BuildError::missing_field("has_full_snapshot"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_event_ts: self.last_event_ts.ok_or_else(|| BuildError::missing_field("last_event_ts"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            started_at: self.started_at.ok_or_else(|| BuildError::missing_field("started_at"))?,
            storage_deleted_at: self.storage_deleted_at,
            surface_id: self.surface_id,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
