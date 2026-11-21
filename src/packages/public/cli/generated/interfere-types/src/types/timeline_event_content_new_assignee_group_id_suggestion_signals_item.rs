pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItem {
    pub kind: TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub score: f64,
    #[serde(default)]
    pub evidence: String,
}

impl TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItem {
    pub fn builder() -> TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder {
        <TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder {
    kind: Option<TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind>,
    score: Option<f64>,
    evidence: Option<String>,
}

impl TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder {
    pub fn kind(mut self, value: TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemKind) -> Self {
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

    /// Consumes the builder and constructs a [`TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder::kind)
    /// - [`score`](TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder::score)
    /// - [`evidence`](TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItemBuilder::evidence)
    pub fn build(self) -> Result<TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItem, BuildError> {
        Ok(TimelineEventContentNewAssigneeGroupIdSuggestionSignalsItem {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            score: self.score.ok_or_else(|| BuildError::missing_field("score"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}
