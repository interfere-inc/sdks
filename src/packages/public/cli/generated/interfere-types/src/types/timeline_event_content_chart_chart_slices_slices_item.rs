pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TimelineEventContentChartChartSlicesSlicesItem {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub percent: f64,
}

impl TimelineEventContentChartChartSlicesSlicesItem {
    pub fn builder() -> TimelineEventContentChartChartSlicesSlicesItemBuilder {
        <TimelineEventContentChartChartSlicesSlicesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartChartSlicesSlicesItemBuilder {
    key: Option<String>,
    label: Option<String>,
    percent: Option<f64>,
}

impl TimelineEventContentChartChartSlicesSlicesItemBuilder {
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

    /// Consumes the builder and constructs a [`TimelineEventContentChartChartSlicesSlicesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](TimelineEventContentChartChartSlicesSlicesItemBuilder::key)
    /// - [`label`](TimelineEventContentChartChartSlicesSlicesItemBuilder::label)
    /// - [`percent`](TimelineEventContentChartChartSlicesSlicesItemBuilder::percent)
    pub fn build(self) -> Result<TimelineEventContentChartChartSlicesSlicesItem, BuildError> {
        Ok(TimelineEventContentChartChartSlicesSlicesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            percent: self.percent.ok_or_else(|| BuildError::missing_field("percent"))?,
        })
    }
}
