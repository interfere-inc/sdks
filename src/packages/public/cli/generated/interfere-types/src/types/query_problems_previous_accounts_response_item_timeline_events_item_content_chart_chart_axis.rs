pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub axis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisAxis>,
    #[serde(default)]
    pub buckets: Vec<i64>,
    #[serde(default)]
    pub label: String,
    #[serde(rename = "markerLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markers: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem>>,
    #[serde(default)]
    pub series: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stacked: Option<bool>,
    pub r#type: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisType,
    pub unit: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisUnit,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder {
    axis: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisAxis>,
    buckets: Option<Vec<i64>>,
    label: Option<String>,
    marker_label: Option<String>,
    markers: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem>>,
    series: Option<Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem>>,
    stacked: Option<bool>,
    r#type: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisType>,
    unit: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisUnit>,
    value: Option<f64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder {
    pub fn axis(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisAxis) -> Self {
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

    pub fn markers(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisMarkersItem>) -> Self {
        self.markers = Some(value);
        self
    }

    pub fn series(mut self, value: Vec<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisSeriesItem>) -> Self {
        self.series = Some(value);
        self
    }

    pub fn stacked(mut self, value: bool) -> Self {
        self.stacked = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn unit(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisUnit) -> Self {
        self.unit = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis`].
    /// This method will fail if any of the following fields are not set:
    /// - [`buckets`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder::buckets)
    /// - [`label`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder::label)
    /// - [`series`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder::series)
    /// - [`r#type`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder::r#type)
    /// - [`unit`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxisBuilder::unit)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChartAxis {
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
