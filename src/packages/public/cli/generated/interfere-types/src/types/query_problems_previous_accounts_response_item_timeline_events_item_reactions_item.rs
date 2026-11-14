pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem {
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(rename = "timelineEventId")]
    #[serde(default)]
    pub timeline_event_id: String,
    #[serde(rename = "userId")]
    #[serde(default)]
    pub user_id: String,
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

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder {
    emoji: Option<String>,
    id: Option<String>,
    organization_id: Option<String>,
    timeline_event_id: Option<String>,
    user_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder {
    pub fn emoji(mut self, value: impl Into<String>) -> Self {
        self.emoji = Some(value.into());
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

    pub fn timeline_event_id(mut self, value: impl Into<String>) -> Self {
        self.timeline_event_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
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

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`emoji`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::emoji)
    /// - [`id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::id)
    /// - [`organization_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::organization_id)
    /// - [`timeline_event_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::timeline_event_id)
    /// - [`user_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::user_id)
    /// - [`created_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::created_at)
    /// - [`updated_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItemBuilder::updated_at)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReactionsItem {
            emoji: self.emoji.ok_or_else(|| BuildError::missing_field("emoji"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            timeline_event_id: self.timeline_event_id.ok_or_else(|| BuildError::missing_field("timeline_event_id"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
