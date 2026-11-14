pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurfaceEventsTimeseriesResponse {
    #[serde(rename = "unavailableLinkedServiceSignals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unavailable_linked_service_signals: Option<Vec<SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem>>,
    pub window: SurfaceEventsTimeseriesResponseWindow,
    #[serde(rename = "bucketRows")]
    #[serde(default)]
    pub bucket_rows: Vec<SurfaceEventsTimeseriesResponseBucketRowsItem>,
    #[serde(default)]
    pub buckets: Vec<DateTime<FixedOffset>>,
    #[serde(default)]
    pub series: SurfaceEventsTimeseriesResponseSeries,
}

impl SurfaceEventsTimeseriesResponse {
    pub fn builder() -> SurfaceEventsTimeseriesResponseBuilder {
        <SurfaceEventsTimeseriesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceEventsTimeseriesResponseBuilder {
    unavailable_linked_service_signals: Option<Vec<SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem>>,
    window: Option<SurfaceEventsTimeseriesResponseWindow>,
    bucket_rows: Option<Vec<SurfaceEventsTimeseriesResponseBucketRowsItem>>,
    buckets: Option<Vec<DateTime<FixedOffset>>>,
    series: Option<SurfaceEventsTimeseriesResponseSeries>,
}

impl SurfaceEventsTimeseriesResponseBuilder {
    pub fn unavailable_linked_service_signals(mut self, value: Vec<SurfaceEventsTimeseriesResponseUnavailableLinkedServiceSignalsItem>) -> Self {
        self.unavailable_linked_service_signals = Some(value);
        self
    }

    pub fn window(mut self, value: SurfaceEventsTimeseriesResponseWindow) -> Self {
        self.window = Some(value);
        self
    }

    pub fn bucket_rows(mut self, value: Vec<SurfaceEventsTimeseriesResponseBucketRowsItem>) -> Self {
        self.bucket_rows = Some(value);
        self
    }

    pub fn buckets(mut self, value: Vec<DateTime<FixedOffset>>) -> Self {
        self.buckets = Some(value);
        self
    }

    pub fn series(mut self, value: SurfaceEventsTimeseriesResponseSeries) -> Self {
        self.series = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurfaceEventsTimeseriesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`window`](SurfaceEventsTimeseriesResponseBuilder::window)
    /// - [`bucket_rows`](SurfaceEventsTimeseriesResponseBuilder::bucket_rows)
    /// - [`buckets`](SurfaceEventsTimeseriesResponseBuilder::buckets)
    /// - [`series`](SurfaceEventsTimeseriesResponseBuilder::series)
    pub fn build(self) -> Result<SurfaceEventsTimeseriesResponse, BuildError> {
        Ok(SurfaceEventsTimeseriesResponse {
            unavailable_linked_service_signals: self.unavailable_linked_service_signals,
            window: self.window.ok_or_else(|| BuildError::missing_field("window"))?,
            bucket_rows: self.bucket_rows.ok_or_else(|| BuildError::missing_field("bucket_rows"))?,
            buckets: self.buckets.ok_or_else(|| BuildError::missing_field("buckets"))?,
            series: self.series.ok_or_else(|| BuildError::missing_field("series"))?,
        })
    }
}
