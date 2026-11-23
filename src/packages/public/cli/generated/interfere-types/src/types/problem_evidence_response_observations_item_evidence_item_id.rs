pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemId {
    #[serde(default)]
    pub id: String,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemIdKind,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemId {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder {
    id: Option<String>,
    content_digest: Option<String>,
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemIdKind>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemIdKind) -> Self {
        self.kind = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder::id)
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder::content_digest)
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemIdBuilder::kind)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemId, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemId {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
