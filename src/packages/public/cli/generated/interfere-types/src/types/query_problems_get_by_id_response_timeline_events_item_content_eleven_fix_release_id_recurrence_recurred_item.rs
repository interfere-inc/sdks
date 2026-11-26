pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "errorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_seen_at: f64,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItemBuilder {
    id: Option<String>,
    error_name: Option<String>,
    route: Option<String>,
    message: Option<String>,
    last_seen_at: Option<f64>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn error_name(mut self, value: impl Into<String>) -> Self {
        self.error_name = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItemBuilder::id)
    /// - [`last_seen_at`](QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItemBuilder::last_seen_at)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentElevenFixReleaseIdRecurrenceRecurredItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            error_name: self.error_name,
            route: self.route,
            message: self.message,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
        })
    }
}
