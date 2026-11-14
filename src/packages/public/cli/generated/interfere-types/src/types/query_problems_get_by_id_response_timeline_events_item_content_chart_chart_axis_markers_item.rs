pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItem {
    #[serde(default)]
    pub index: i64,
    #[serde(default)]
    pub label: String,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItemBuilder {
    index: Option<i64>,
    label: Option<String>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItemBuilder {
    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`index`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItemBuilder::index)
    /// - [`label`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItemBuilder::label)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisMarkersItem {
            index: self.index.ok_or_else(|| BuildError::missing_field("index"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
        })
    }
}
