pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SurfaceActivityResponseValue {
    #[serde(rename = "hasRecentEvents")]
    #[serde(default)]
    pub has_recent_events: bool,
    #[serde(default)]
    pub sparkline: Vec<f64>,
    pub status: SurfaceActivityResponseValueStatus,
}

impl SurfaceActivityResponseValue {
    pub fn builder() -> SurfaceActivityResponseValueBuilder {
        <SurfaceActivityResponseValueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceActivityResponseValueBuilder {
    has_recent_events: Option<bool>,
    sparkline: Option<Vec<f64>>,
    status: Option<SurfaceActivityResponseValueStatus>,
}

impl SurfaceActivityResponseValueBuilder {
    pub fn has_recent_events(mut self, value: bool) -> Self {
        self.has_recent_events = Some(value);
        self
    }

    pub fn sparkline(mut self, value: Vec<f64>) -> Self {
        self.sparkline = Some(value);
        self
    }

    pub fn status(mut self, value: SurfaceActivityResponseValueStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurfaceActivityResponseValue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_recent_events`](SurfaceActivityResponseValueBuilder::has_recent_events)
    /// - [`sparkline`](SurfaceActivityResponseValueBuilder::sparkline)
    /// - [`status`](SurfaceActivityResponseValueBuilder::status)
    pub fn build(self) -> Result<SurfaceActivityResponseValue, BuildError> {
        Ok(SurfaceActivityResponseValue {
            has_recent_events: self.has_recent_events.ok_or_else(|| BuildError::missing_field("has_recent_events"))?,
            sparkline: self.sparkline.ok_or_else(|| BuildError::missing_field("sparkline"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
