pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetServiceReportableEnvironmentResponse {
    #[serde(default)]
    pub ok: String,
}

impl SetServiceReportableEnvironmentResponse {
    pub fn builder() -> SetServiceReportableEnvironmentResponseBuilder {
        <SetServiceReportableEnvironmentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetServiceReportableEnvironmentResponseBuilder {
    ok: Option<String>,
}

impl SetServiceReportableEnvironmentResponseBuilder {
    pub fn ok(mut self, value: impl Into<String>) -> Self {
        self.ok = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetServiceReportableEnvironmentResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ok`](SetServiceReportableEnvironmentResponseBuilder::ok)
    pub fn build(self) -> Result<SetServiceReportableEnvironmentResponse, BuildError> {
        Ok(SetServiceReportableEnvironmentResponse {
            ok: self.ok.ok_or_else(|| BuildError::missing_field("ok"))?,
        })
    }
}
