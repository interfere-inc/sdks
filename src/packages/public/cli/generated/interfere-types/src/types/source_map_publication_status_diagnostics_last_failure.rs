pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SourceMapPublicationStatusDiagnosticsLastFailure {
    #[serde(default)]
    pub attempt: i64,
    #[serde(rename = "atMs")]
    #[serde(default)]
    pub at_ms: i64,
    pub phase: SourceMapPublicationStatusDiagnosticsLastFailurePhase,
    pub kind: SourceMapPublicationStatusDiagnosticsLastFailureKind,
}

impl SourceMapPublicationStatusDiagnosticsLastFailure {
    pub fn builder() -> SourceMapPublicationStatusDiagnosticsLastFailureBuilder {
        <SourceMapPublicationStatusDiagnosticsLastFailureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SourceMapPublicationStatusDiagnosticsLastFailureBuilder {
    attempt: Option<i64>,
    at_ms: Option<i64>,
    phase: Option<SourceMapPublicationStatusDiagnosticsLastFailurePhase>,
    kind: Option<SourceMapPublicationStatusDiagnosticsLastFailureKind>,
}

impl SourceMapPublicationStatusDiagnosticsLastFailureBuilder {
    pub fn attempt(mut self, value: i64) -> Self {
        self.attempt = Some(value);
        self
    }

    pub fn at_ms(mut self, value: i64) -> Self {
        self.at_ms = Some(value);
        self
    }

    pub fn phase(mut self, value: SourceMapPublicationStatusDiagnosticsLastFailurePhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn kind(mut self, value: SourceMapPublicationStatusDiagnosticsLastFailureKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SourceMapPublicationStatusDiagnosticsLastFailure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attempt`](SourceMapPublicationStatusDiagnosticsLastFailureBuilder::attempt)
    /// - [`at_ms`](SourceMapPublicationStatusDiagnosticsLastFailureBuilder::at_ms)
    /// - [`phase`](SourceMapPublicationStatusDiagnosticsLastFailureBuilder::phase)
    /// - [`kind`](SourceMapPublicationStatusDiagnosticsLastFailureBuilder::kind)
    pub fn build(self) -> Result<SourceMapPublicationStatusDiagnosticsLastFailure, BuildError> {
        Ok(SourceMapPublicationStatusDiagnosticsLastFailure {
            attempt: self.attempt.ok_or_else(|| BuildError::missing_field("attempt"))?,
            at_ms: self.at_ms.ok_or_else(|| BuildError::missing_field("at_ms"))?,
            phase: self.phase.ok_or_else(|| BuildError::missing_field("phase"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
