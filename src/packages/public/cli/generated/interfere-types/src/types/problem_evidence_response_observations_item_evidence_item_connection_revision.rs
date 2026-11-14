pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionKind,
    #[serde(rename = "connectionRevision")]
    #[serde(default)]
    pub connection_revision: i64,
    pub operation: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenance>,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionKind>,
    connection_revision: Option<i64>,
    operation: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperation>,
    provenance: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenance>,
    content_digest: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn connection_revision(mut self, value: i64) -> Self {
        self.connection_revision = Some(value);
        self
    }

    pub fn operation(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn provenance(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionProvenance) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder::kind)
    /// - [`connection_revision`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder::connection_revision)
    /// - [`operation`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder::operation)
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionBuilder::content_digest)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevision {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            connection_revision: self.connection_revision.ok_or_else(|| BuildError::missing_field("connection_revision"))?,
            operation: self.operation.ok_or_else(|| BuildError::missing_field("operation"))?,
            provenance: self.provenance,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
        })
    }
}
