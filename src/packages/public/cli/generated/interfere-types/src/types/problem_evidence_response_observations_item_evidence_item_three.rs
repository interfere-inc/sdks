pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemThree {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemThreeKind,
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemThree {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemThreeBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemThreeBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemThreeKind>,
    content_digest: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemThreeBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemThreeKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemThreeBuilder::kind)
    /// - [`content_digest`](ProblemEvidenceResponseObservationsItemEvidenceItemThreeBuilder::content_digest)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemThree, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemThree {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
        })
    }
}
