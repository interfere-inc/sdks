pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percent: f64,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem {
    pub fn builder() -> QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder {
        <QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    percent: Option<f64>,
}

impl QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn percent(mut self, value: f64) -> Self {
        self.percent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder::key)
    /// - [`label`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder::label)
    /// - [`percent`](QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItemBuilder::percent)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem, BuildError> {
        Ok(QueryProblemsGetByIdResponseTimelineEventsItemContentChartChartSlicesSlicesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            percent: self.percent.ok_or_else(|| BuildError::missing_field("percent"))?,
        })
    }
}
