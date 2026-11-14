pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame {
    #[serde(rename = "evidenceDigest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
    #[serde(rename = "factId")]
    #[serde(default)]
    pub fact_id: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameKind,
    #[serde(rename = "sourceFrame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_frame: Option<ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameBuilder {
    evidence_digest: Option<String>,
    fact_id: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameKind>,
    source_frame: Option<ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameBuilder {
    pub fn evidence_digest(mut self, value: impl Into<String>) -> Self {
        self.evidence_digest = Some(value.into());
        self
    }

    pub fn fact_id(mut self, value: impl Into<String>) -> Self {
        self.fact_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn source_frame(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameSourceFrame) -> Self {
        self.source_frame = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fact_id`](ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameBuilder::fact_id)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrameBuilder::kind)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemSourceFrame {
            evidence_digest: self.evidence_digest,
            fact_id: self.fact_id.ok_or_else(|| BuildError::missing_field("fact_id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            source_frame: self.source_frame,
        })
    }
}
