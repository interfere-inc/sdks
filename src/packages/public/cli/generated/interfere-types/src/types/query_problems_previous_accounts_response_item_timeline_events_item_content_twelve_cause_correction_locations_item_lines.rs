pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLinesBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLinesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLinesBuilder {
    start: Option<serde_json::Value>,
    end: Option<serde_json::Value>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLinesBuilder {
    pub fn start(mut self, value: serde_json::Value) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: serde_json::Value) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines`].
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTwelveCauseCorrectionLocationsItemLines {
            start: self.start,
            end: self.end,
        })
    }
}
