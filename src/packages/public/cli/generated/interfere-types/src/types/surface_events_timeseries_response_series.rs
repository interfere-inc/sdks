pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SurfaceEventsTimeseriesResponseSeries {
    #[serde(default)]
    pub traces: Vec<f64>,
    #[serde(default)]
    pub logs: Vec<f64>,
    #[serde(default)]
    pub metrics: Vec<f64>,
}

impl SurfaceEventsTimeseriesResponseSeries {
    pub fn builder() -> SurfaceEventsTimeseriesResponseSeriesBuilder {
        <SurfaceEventsTimeseriesResponseSeriesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceEventsTimeseriesResponseSeriesBuilder {
    traces: Option<Vec<f64>>,
    logs: Option<Vec<f64>>,
    metrics: Option<Vec<f64>>,
}

impl SurfaceEventsTimeseriesResponseSeriesBuilder {
    pub fn traces(mut self, value: Vec<f64>) -> Self {
        self.traces = Some(value);
        self
    }

    pub fn logs(mut self, value: Vec<f64>) -> Self {
        self.logs = Some(value);
        self
    }

    pub fn metrics(mut self, value: Vec<f64>) -> Self {
        self.metrics = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurfaceEventsTimeseriesResponseSeries`].
    /// This method will fail if any of the following fields are not set:
    /// - [`traces`](SurfaceEventsTimeseriesResponseSeriesBuilder::traces)
    /// - [`logs`](SurfaceEventsTimeseriesResponseSeriesBuilder::logs)
    /// - [`metrics`](SurfaceEventsTimeseriesResponseSeriesBuilder::metrics)
    pub fn build(self) -> Result<SurfaceEventsTimeseriesResponseSeries, BuildError> {
        Ok(SurfaceEventsTimeseriesResponseSeries {
            traces: self.traces.ok_or_else(|| BuildError::missing_field("traces"))?,
            logs: self.logs.ok_or_else(|| BuildError::missing_field("logs"))?,
            metrics: self.metrics.ok_or_else(|| BuildError::missing_field("metrics"))?,
        })
    }
}
