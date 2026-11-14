pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SurfaceEventsTimeseriesResponseBucketRowsItem {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub logs: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub metrics: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub traces: f64,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub start: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub end: DateTime<FixedOffset>,
    #[serde(default)]
    pub index: i64,
    #[serde(rename = "isLatest")]
    #[serde(default)]
    pub is_latest: bool,
    #[serde(rename = "isQuiet")]
    #[serde(default)]
    pub is_quiet: bool,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
}

impl SurfaceEventsTimeseriesResponseBucketRowsItem {
    pub fn builder() -> SurfaceEventsTimeseriesResponseBucketRowsItemBuilder {
        <SurfaceEventsTimeseriesResponseBucketRowsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceEventsTimeseriesResponseBucketRowsItemBuilder {
    logs: Option<f64>,
    metrics: Option<f64>,
    traces: Option<f64>,
    start: Option<DateTime<FixedOffset>>,
    end: Option<DateTime<FixedOffset>>,
    index: Option<i64>,
    is_latest: Option<bool>,
    is_quiet: Option<bool>,
    total: Option<f64>,
}

impl SurfaceEventsTimeseriesResponseBucketRowsItemBuilder {
    pub fn logs(mut self, value: f64) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn metrics(mut self, value: f64) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn traces(mut self, value: f64) -> Self {
        self.traces = Some(value);
        self
    }

    pub fn start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.start = Some(value);
        self
    }

    pub fn end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.end = Some(value);
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn is_latest(mut self, value: bool) -> Self {
        self.is_latest = Some(value);
        self
    }

    pub fn is_quiet(mut self, value: bool) -> Self {
        self.is_quiet = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurfaceEventsTimeseriesResponseBucketRowsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`logs`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::logs)
    /// - [`metrics`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::metrics)
    /// - [`traces`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::traces)
    /// - [`start`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::start)
    /// - [`end`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::end)
    /// - [`index`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::index)
    /// - [`is_latest`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::is_latest)
    /// - [`is_quiet`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::is_quiet)
    /// - [`total`](SurfaceEventsTimeseriesResponseBucketRowsItemBuilder::total)
    pub fn build(self) -> Result<SurfaceEventsTimeseriesResponseBucketRowsItem, BuildError> {
        Ok(SurfaceEventsTimeseriesResponseBucketRowsItem {
            logs: self.logs.ok_or_else(|| BuildError::missing_field("logs"))?,
            metrics: self.metrics.ok_or_else(|| BuildError::missing_field("metrics"))?,
            traces: self.traces.ok_or_else(|| BuildError::missing_field("traces"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            index: self.index.ok_or_else(|| BuildError::missing_field("index"))?,
            is_latest: self.is_latest.ok_or_else(|| BuildError::missing_field("is_latest"))?,
            is_quiet: self.is_quiet.ok_or_else(|| BuildError::missing_field("is_quiet"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
