pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TimelineEventContentChartChartAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<TimelineEventContentChartChartAxisAxis>,
    #[serde(default)]
    pub buckets: Vec<i64>,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "markerLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<TimelineEventContentChartChartAxisMarkersItem>>,
    #[serde(default)]
    pub series: Vec<TimelineEventContentChartChartAxisSeriesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacked: Option<bool>,
    pub r#type: TimelineEventContentChartChartAxisType,
    pub unit: TimelineEventContentChartChartAxisUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl TimelineEventContentChartChartAxis {
    pub fn builder() -> TimelineEventContentChartChartAxisBuilder {
        <TimelineEventContentChartChartAxisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentChartChartAxisBuilder {
    axis: Option<TimelineEventContentChartChartAxisAxis>,
    buckets: Option<Vec<i64>>,
    label: Option<String>,
    marker_label: Option<String>,
    markers: Option<Vec<TimelineEventContentChartChartAxisMarkersItem>>,
    series: Option<Vec<TimelineEventContentChartChartAxisSeriesItem>>,
    stacked: Option<bool>,
    r#type: Option<TimelineEventContentChartChartAxisType>,
    unit: Option<TimelineEventContentChartChartAxisUnit>,
    value: Option<f64>,
}

impl TimelineEventContentChartChartAxisBuilder {
    pub fn axis(mut self, value: TimelineEventContentChartChartAxisAxis) -> Self {
        self.axis = Some(value);
        self
    }

    pub fn buckets(mut self, value: Vec<i64>) -> Self {
        self.buckets = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn marker_label(mut self, value: impl Into<String>) -> Self {
        self.marker_label = Some(value.into());
        self
    }

    pub fn markers(mut self, value: Vec<TimelineEventContentChartChartAxisMarkersItem>) -> Self {
        self.markers = Some(value);
        self
    }

    pub fn series(mut self, value: Vec<TimelineEventContentChartChartAxisSeriesItem>) -> Self {
        self.series = Some(value);
        self
    }

    pub fn stacked(mut self, value: bool) -> Self {
        self.stacked = Some(value);
        self
    }

    pub fn r#type(mut self, value: TimelineEventContentChartChartAxisType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit(mut self, value: TimelineEventContentChartChartAxisUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentChartChartAxis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`buckets`](TimelineEventContentChartChartAxisBuilder::buckets)
    /// - [`label`](TimelineEventContentChartChartAxisBuilder::label)
    /// - [`series`](TimelineEventContentChartChartAxisBuilder::series)
    /// - [`r#type`](TimelineEventContentChartChartAxisBuilder::r#type)
    /// - [`unit`](TimelineEventContentChartChartAxisBuilder::unit)
    pub fn build(self) -> Result<TimelineEventContentChartChartAxis, BuildError> {
        Ok(TimelineEventContentChartChartAxis {
            axis: self.axis,
            buckets: self.buckets.ok_or_else(|| BuildError::missing_field("buckets"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            marker_label: self.marker_label,
            markers: self.markers,
            series: self.series.ok_or_else(|| BuildError::missing_field("series"))?,
            stacked: self.stacked,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            value: self.value,
        })
    }
}
