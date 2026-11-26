pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLines {
    #[serde(default)]
    pub start: i64,
    #[serde(default)]
    pub end: i64,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLines {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder {
    start: Option<i64>,
    end: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder {
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLines`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder::start)
    /// - [`end`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder::end)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLines, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentCorrectionCorrectionLocationsItemLines {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}
