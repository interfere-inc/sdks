pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion {
    #[serde(default)]
    pub version: String,
    #[serde(rename = "suggestionId")]
    #[serde(default)]
    pub suggestion_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub confidence: f64,
    #[serde(default)]
    pub signals: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionSignalsItem>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder {
    version: Option<String>,
    suggestion_id: Option<String>,
    confidence: Option<f64>,
    signals: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionSignalsItem>>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder {
    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    pub fn suggestion_id(mut self, value: impl Into<String>) -> Self {
        self.suggestion_id = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: f64) -> Self {
        self.confidence = Some(value);
        self
    }

    pub fn signals(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionSignalsItem>) -> Self {
        self.signals = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder::version)
    /// - [`suggestion_id`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder::suggestion_id)
    /// - [`confidence`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder::confidence)
    /// - [`signals`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestionBuilder::signals)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentNewAssigneeGroupIdSuggestion {
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
            suggestion_id: self.suggestion_id.ok_or_else(|| BuildError::missing_field("suggestion_id"))?,
            confidence: self.confidence.ok_or_else(|| BuildError::missing_field("confidence"))?,
            signals: self.signals.ok_or_else(|| BuildError::missing_field("signals"))?,
        })
    }
}
