pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "surfaceChangeId")]
    #[serde(default)]
    pub surface_change_id: String,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
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
    #[serde(rename = "surfaceChange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_change: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder {
    id: Option<String>,
    organization_id: Option<String>,
    surface_change_id: Option<String>,
    timeline_event_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    surface_change: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn surface_change_id(mut self, value: impl Into<String>) -> Self {
        self.surface_change_id = Some(value.into());
        self
    }

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
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

    pub fn surface_change(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange) -> Self {
        self.surface_change = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder::id)
    /// - [`organization_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder::organization_id)
    /// - [`surface_change_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder::surface_change_id)
    /// - [`timeline_event_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder::timeline_event_id)
    /// - [`created_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder::created_at)
    /// - [`updated_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            surface_change_id: self.surface_change_id.ok_or_else(|| BuildError::missing_field("surface_change_id"))?,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            surface_change: self.surface_change,
        })
    }
}
