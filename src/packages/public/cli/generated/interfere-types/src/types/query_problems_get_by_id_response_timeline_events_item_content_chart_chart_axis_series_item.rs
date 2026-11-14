pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub values: Vec<Option<f64>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    values: Option<Vec<Option<f64>>>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<Option<f64>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder::key)
    /// - [`label`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder::label)
    /// - [`values`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItemBuilder::values)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartAxisSeriesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
