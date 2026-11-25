pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSeven {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemSevenKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSeven {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemSevenBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemSevenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSevenBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemSevenKind>,
    problem_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSevenBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemSevenKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemSeven`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemSevenBuilder::kind)
    /// - [`problem_id`](ProblemEvidenceResponseObservationsItemEvidenceItemSevenBuilder::problem_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemSeven, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemSeven {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
