pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorKind>,
    cursor: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursorBuilder::kind)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemConnectionRevisionOperationCursor {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            cursor: self.cursor,
        })
    }
}
