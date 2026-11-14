pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for surfaceEventsTimeseries
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurfaceEventsTimeseriesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window: Option<SurfaceEventsTimeseriesRequestWindow>,
}

impl SurfaceEventsTimeseriesQueryRequest {
    pub fn builder() -> SurfaceEventsTimeseriesQueryRequestBuilder {
        <SurfaceEventsTimeseriesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceEventsTimeseriesQueryRequestBuilder {
    environment: Option<String>,
    window: Option<SurfaceEventsTimeseriesRequestWindow>,
}

impl SurfaceEventsTimeseriesQueryRequestBuilder {
    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    pub fn window(mut self, value: SurfaceEventsTimeseriesRequestWindow) -> Self {
        self.window = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SurfaceEventsTimeseriesQueryRequest`].
    pub fn build(self) -> Result<SurfaceEventsTimeseriesQueryRequest, BuildError> {
        Ok(SurfaceEventsTimeseriesQueryRequest {
            environment: self.environment,
            window: self.window,
        })
    }
}

