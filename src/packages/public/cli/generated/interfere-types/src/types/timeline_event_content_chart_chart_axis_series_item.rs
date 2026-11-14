pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChartChartAxisSeriesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub values: Vec<Option<f64>>,
}

impl TimelineEventContentChartChartAxisSeriesItem {
    pub fn builder() -> TimelineEventContentChartChartAxisSeriesItemBuilder {
        <TimelineEventContentChartChartAxisSeriesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartChartAxisSeriesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    values: Option<Vec<Option<f64>>>,
}

impl TimelineEventContentChartChartAxisSeriesItemBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentChartChartAxisSeriesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TimelineEventContentChartChartAxisSeriesItemBuilder::key)
    /// - [`label`](TimelineEventContentChartChartAxisSeriesItemBuilder::label)
    /// - [`values`](TimelineEventContentChartChartAxisSeriesItemBuilder::values)
    pub fn build(self) -> Result<TimelineEventContentChartChartAxisSeriesItem, BuildError> {
        Ok(TimelineEventContentChartChartAxisSeriesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            values: self.values.ok_or_else(|| BuildError::missing_field("values"))?,
        })
    }
}
