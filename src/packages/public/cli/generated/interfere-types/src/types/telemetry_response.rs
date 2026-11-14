pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// TelemetryResponse
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TelemetryResponse {
    #[serde(default)]
    pub ok: bool,
}

impl TelemetryResponse {
    pub fn builder() -> TelemetryResponseBuilder {
        <TelemetryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TelemetryResponseBuilder {
    ok: Option<bool>,
}

impl TelemetryResponseBuilder {
    pub fn ok(mut self, value: bool) -> Self {
        self.ok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TelemetryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](TelemetryResponseBuilder::ok)
    pub fn build(self) -> Result<TelemetryResponse, BuildError> {
        Ok(TelemetryResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
