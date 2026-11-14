pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor {
    #[serde(rename = "startTimeUnixNano")]
    #[serde(default)]
    pub start_time_unix_nano: String,
    #[serde(rename = "spanId")]
    #[serde(default)]
    pub span_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursorBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursorBuilder {
    start_time_unix_nano: Option<String>,
    span_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursorBuilder {
    pub fn start_time_unix_nano(mut self, value: impl Into<String>) -> Self {
        self.start_time_unix_nano = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_time_unix_nano`](ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursorBuilder::start_time_unix_nano)
    /// - [`span_id`](ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursorBuilder::span_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor {
            start_time_unix_nano: self.start_time_unix_nano.ok_or_else(|| BuildError::missing_field("start_time_unix_nano"))?,
            span_id: self.span_id.ok_or_else(|| BuildError::missing_field("span_id"))?,
        })
    }
}
