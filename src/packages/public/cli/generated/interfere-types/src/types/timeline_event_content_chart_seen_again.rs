pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentChartSeenAgain {
    #[serde(default)]
    pub at: i64,
    #[serde(default)]
    pub count: i64,
    #[serde(rename = "operationId")]
    #[serde(default)]
    pub operation_id: String,
}

impl TimelineEventContentChartSeenAgain {
    pub fn builder() -> TimelineEventContentChartSeenAgainBuilder {
        <TimelineEventContentChartSeenAgainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartSeenAgainBuilder {
    at: Option<i64>,
    count: Option<i64>,
    operation_id: Option<String>,
}

impl TimelineEventContentChartSeenAgainBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentChartSeenAgain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`at`](TimelineEventContentChartSeenAgainBuilder::at)
    /// - [`count`](TimelineEventContentChartSeenAgainBuilder::count)
    /// - [`operation_id`](TimelineEventContentChartSeenAgainBuilder::operation_id)
    pub fn build(self) -> Result<TimelineEventContentChartSeenAgain, BuildError> {
        Ok(TimelineEventContentChartSeenAgain {
            at: self.at.ok_or_else(|| BuildError::missing_field("at"))?,
            count: self.count.ok_or_else(|| BuildError::missing_field("count"))?,
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
        })
    }
}
