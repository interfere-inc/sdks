pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemFour {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemFourKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemFour {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemFourBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemFourBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemFourBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemFourKind>,
    problem_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemFourBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemFourKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemFour`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemFourBuilder::kind)
    /// - [`problem_id`](ProblemEvidenceResponseObservationsItemEvidenceItemFourBuilder::problem_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemFour, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemFour {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
