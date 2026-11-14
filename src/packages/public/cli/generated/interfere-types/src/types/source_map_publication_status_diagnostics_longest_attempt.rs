pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SourceMapPublicationStatusDiagnosticsLongestAttempt {
    #[serde(default)]
    pub attempt: i64,
    #[serde(rename = "durationMs")]
    #[serde(default)]
    pub duration_ms: i64,
}

impl SourceMapPublicationStatusDiagnosticsLongestAttempt {
    pub fn builder() -> SourceMapPublicationStatusDiagnosticsLongestAttemptBuilder {
        <SourceMapPublicationStatusDiagnosticsLongestAttemptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapPublicationStatusDiagnosticsLongestAttemptBuilder {
    attempt: Option<i64>,
    duration_ms: Option<i64>,
}

impl SourceMapPublicationStatusDiagnosticsLongestAttemptBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapPublicationStatusDiagnosticsLongestAttempt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempt`](SourceMapPublicationStatusDiagnosticsLongestAttemptBuilder::attempt)
    /// - [`duration_ms`](SourceMapPublicationStatusDiagnosticsLongestAttemptBuilder::duration_ms)
    pub fn build(self) -> Result<SourceMapPublicationStatusDiagnosticsLongestAttempt, BuildError> {
        Ok(SourceMapPublicationStatusDiagnosticsLongestAttempt {
            attempt: self.attempt.ok_or_else(|| BuildError::missing_field("attempt"))?,
            duration_ms: self.duration_ms.ok_or_else(|| BuildError::missing_field("duration_ms"))?,
        })
    }
}
