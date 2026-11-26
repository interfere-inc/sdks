pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLines {
    #[serde(default)]
    pub start: i64,
    #[serde(default)]
    pub end: i64,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLines {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder {
    start: Option<i64>,
    end: Option<i64>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder {
    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLines`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder::start)
    /// - [`end`](QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLinesBuilder::end)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLines, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentCorrectionCorrectionLocationsItemLines {
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
        })
    }
}
