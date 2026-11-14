pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItem {
    pub kind: QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemKind,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
    #[serde(default)]
    pub evidence: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder {
    kind: Option<QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemKind>,
    score: Option<f64>,
    evidence: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder {
    pub fn kind(mut self, value: QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn score(mut self, value: f64) -> Self {
        self.score = Some(value);
        self
    }

    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder::kind)
    /// - [`score`](QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder::score)
    /// - [`evidence`](QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItemBuilder::evidence)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentNewAssigneeUserIdSuggestionSignalsItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}
