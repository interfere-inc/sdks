pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    #[serde(rename = "evidenceDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
    #[serde(rename = "factId")]
    #[serde(default)]
    pub fact_id: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountKind,
    #[serde(default)]
    pub limit: i64,
    #[serde(rename = "recordCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<i64>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder {
    content_digest: Option<String>,
    evidence_digest: Option<String>,
    fact_id: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountKind>,
    limit: Option<i64>,
    record_count: Option<i64>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder {
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

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn record_count(mut self, value: i64) -> Self {
        self.record_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder::content_digest)
    /// - [`fact_id`](ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder::fact_id)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder::kind)
    /// - [`limit`](ProblemEvidenceResponseObservationsItemEvidenceItemRecordCountBuilder::limit)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemRecordCount {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            evidence_digest: self.evidence_digest,
            fact_id: self.fact_id.ok_or_else(|| BuildError::missing_field("fact_id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            limit: self.limit.ok_or_else(|| BuildError::missing_field("limit"))?,
            record_count: self.record_count,
        })
    }
}
