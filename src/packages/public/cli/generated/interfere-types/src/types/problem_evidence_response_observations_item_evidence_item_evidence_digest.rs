pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    #[serde(rename = "evidenceDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
    #[serde(rename = "factId")]
    #[serde(default)]
    pub fact_id: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestKind,
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "spanCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_cursor: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projection: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestProjection>,
    #[serde(rename = "spanIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_ids: Option<Vec<String>>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder {
    content_digest: Option<String>,
    evidence_digest: Option<String>,
    fact_id: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestKind>,
    limit: Option<i64>,
    span_cursor: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor>,
    projection: Option<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestProjection>,
    span_ids: Option<Vec<String>>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder {
    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn evidence_digest(mut self, value: impl Into<String>) -> Self {
        self.evidence_digest = Some(value.into());
        self
    }

    pub fn fact_id(mut self, value: impl Into<String>) -> Self {
        self.fact_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn span_cursor(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestSpanCursor) -> Self {
        self.span_cursor = Some(value);
        self
    }

    pub fn projection(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestProjection) -> Self {
        self.projection = Some(value);
        self
    }

    pub fn span_ids(mut self, value: Vec<String>) -> Self {
        self.span_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder::content_digest)
    /// - [`fact_id`](ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder::fact_id)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder::kind)
    /// - [`limit`](ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigestBuilder::limit)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemEvidenceDigest {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            evidence_digest: self.evidence_digest,
            fact_id: self.fact_id.ok_or_else(|| BuildError::missing_field("fact_id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            span_cursor: self.span_cursor,
            projection: self.projection,
            span_ids: self.span_ids,
        })
    }
}
