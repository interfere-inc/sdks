pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindKind,
    #[serde(default)]
    pub uri: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindKind>,
    uri: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn uri(mut self, value: impl Into<String>) -> Self {
        self.uri = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindBuilder::kind)
    /// - [`uri`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKindBuilder::uri)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationKind {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            uri: self.uri.ok_or_else(|| BuildError::missing_field("uri"))?,
        })
    }
}
