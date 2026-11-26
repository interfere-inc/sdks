pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemLimit {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemLimitKind,
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "minSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_severity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "windowEnd")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub window_end: DateTime<FixedOffset>,
    #[serde(rename = "windowStart")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub window_start: DateTime<FixedOffset>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemLimit {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder {
    content_digest: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemLimitKind>,
    limit: Option<i64>,
    min_severity: Option<i64>,
    text: Option<String>,
    window_end: Option<DateTime<FixedOffset>>,
    window_start: Option<DateTime<FixedOffset>>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder {
    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemLimitKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn min_severity(mut self, value: i64) -> Self {
        self.min_severity = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn window_end(mut self, value: DateTime<FixedOffset>) -> Self {
        self.window_end = Some(value);
        self
    }

    pub fn window_start(mut self, value: DateTime<FixedOffset>) -> Self {
        self.window_start = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemLimit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder::content_digest)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder::kind)
    /// - [`limit`](ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder::limit)
    /// - [`window_end`](ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder::window_end)
    /// - [`window_start`](ProblemEvidenceResponseObservationsItemEvidenceItemLimitBuilder::window_start)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemLimit, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemLimit {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            min_severity: self.min_severity,
            text: self.text,
            window_end: self.window_end.ok_or_else(|| BuildError::missing_field("window_end"))?,
            window_start: self.window_start.ok_or_else(|| BuildError::missing_field("window_start"))?,
        })
    }
}
