pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxis {
    #[serde(default)]
    pub end: String,
    #[serde(default)]
    pub start: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxis {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxisBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxisBuilder {
    end: Option<String>,
    start: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxisBuilder {
    pub fn end(mut self, value: impl Into<String>) -> Self {
        self.end = Some(value.into());
        self
    }

    pub fn start(mut self, value: impl Into<String>) -> Self {
        self.start = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`end`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxisBuilder::end)
    /// - [`start`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxisBuilder::start)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxis, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisAxis {
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
        })
    }
}
