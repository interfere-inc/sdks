pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain {
    #[serde(default)]
    pub at: i64,
    #[serde(default)]
    pub count: i64,
    #[serde(rename = "operationId")]
    #[serde(default)]
    pub operation_id: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder {
    at: Option<i64>,
    count: Option<i64>,
    operation_id: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder {
    pub fn at(mut self, value: i64) -> Self {
        self.at = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn operation_id(mut self, value: impl Into<String>) -> Self {
        self.operation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder::at)
    /// - [`count`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder::count)
    /// - [`operation_id`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgainBuilder::operation_id)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartSeenAgain {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
        })
    }
}
