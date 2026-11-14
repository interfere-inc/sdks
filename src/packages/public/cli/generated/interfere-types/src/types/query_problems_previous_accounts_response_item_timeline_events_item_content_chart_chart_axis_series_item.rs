pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub values: Vec<Option<serde_json::Value>>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    values: Option<Vec<Option<serde_json::Value>>>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn values(mut self, value: Vec<Option<serde_json::Value>>) -> Self {
        self.values = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder::key)
    /// - [`label`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder::label)
    /// - [`values`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItemBuilder::values)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
