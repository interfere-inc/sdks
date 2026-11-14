pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange {
    #[serde(rename = "causedById")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caused_by_id: Option<String>,
    #[serde(rename = "displayDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_description: Option<String>,
    #[serde(rename = "displayLabel")]
    #[serde(default)]
    pub display_label: String,
    #[serde(rename = "displayUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
    #[serde(rename = "occurredAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub occurred_at: f64,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(default)]
    pub payload: HashMap<String, Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangePayloadValue>>,
    #[serde(default)]
    pub provider: String,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
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
    pub release: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeRelease>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder {
    caused_by_id: Option<String>,
    display_description: Option<String>,
    display_label: Option<String>,
    display_url: Option<String>,
    id: Option<String>,
    kind: Option<String>,
    occurred_at: Option<f64>,
    organization_id: Option<String>,
    payload: Option<HashMap<String, Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangePayloadValue>>>,
    provider: Option<String>,
    surface_id: Option<String>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    release: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeRelease>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder {
    pub fn caused_by_id(mut self, value: impl Into<String>) -> Self {
        self.caused_by_id = Some(value.into());
        self
    }

    pub fn display_description(mut self, value: impl Into<String>) -> Self {
        self.display_description = Some(value.into());
        self
    }

    pub fn display_label(mut self, value: impl Into<String>) -> Self {
        self.display_label = Some(value.into());
        self
    }

    pub fn display_url(mut self, value: impl Into<String>) -> Self {
        self.display_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: f64) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangePayloadValue>>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn provider(mut self, value: impl Into<String>) -> Self {
        self.provider = Some(value.into());
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
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

    pub fn release(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeRelease) -> Self {
        self.release = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_label`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::display_label)
    /// - [`id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::id)
    /// - [`kind`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::kind)
    /// - [`occurred_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::occurred_at)
    /// - [`organization_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::organization_id)
    /// - [`payload`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::payload)
    /// - [`provider`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::provider)
    /// - [`created_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::created_at)
    /// - [`updated_at`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChangeBuilder::updated_at)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemReferencesItemSurfaceChange {
            caused_by_id: self.caused_by_id,
            display_description: self.display_description,
            display_label: self.display_label.ok_or_else(|| BuildError::missing_field("display_label"))?,
            display_url: self.display_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            occurred_at: self.occurred_at.ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            surface_id: self.surface_id,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            release: self.release,
        })
    }
}
