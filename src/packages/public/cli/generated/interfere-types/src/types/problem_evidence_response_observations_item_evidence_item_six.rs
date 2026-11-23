pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSix {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemSixKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSix {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemSixBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemSixBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemSixBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemSixKind>,
    problem_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemSixBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemSixKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemSix`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemSixBuilder::kind)
    /// - [`problem_id`](ProblemEvidenceResponseObservationsItemEvidenceItemSixBuilder::problem_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemSix, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemSix {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
