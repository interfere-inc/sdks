pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentNewAssigneeUserIdSuggestionSignalsItem {
    pub kind: TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemKind,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
    #[serde(default)]
    pub evidence: String,
}

impl TimelineEventContentNewAssigneeUserIdSuggestionSignalsItem {
    pub fn builder() -> TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder {
        <TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder {
    kind: Option<TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemKind>,
    score: Option<f64>,
    evidence: Option<String>,
}

impl TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder {
    pub fn kind(mut self, value: TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentNewAssigneeUserIdSuggestionSignalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder::kind)
    /// - [`score`](TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder::score)
    /// - [`evidence`](TimelineEventContentNewAssigneeUserIdSuggestionSignalsItemBuilder::evidence)
    pub fn build(self) -> Result<TimelineEventContentNewAssigneeUserIdSuggestionSignalsItem, BuildError> {
        Ok(TimelineEventContentNewAssigneeUserIdSuggestionSignalsItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}
