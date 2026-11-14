pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemFive {
    pub kind: ProblemEvidenceResponseObservationsItemEvidenceItemFiveKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemFive {
    pub fn builder() -> ProblemEvidenceResponseObservationsItemEvidenceItemFiveBuilder {
        <ProblemEvidenceResponseObservationsItemEvidenceItemFiveBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemEvidenceResponseObservationsItemEvidenceItemFiveBuilder {
    kind: Option<ProblemEvidenceResponseObservationsItemEvidenceItemFiveKind>,
    problem_id: Option<String>,
}

impl ProblemEvidenceResponseObservationsItemEvidenceItemFiveBuilder {
    pub fn kind(mut self, value: ProblemEvidenceResponseObservationsItemEvidenceItemFiveKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemEvidenceResponseObservationsItemEvidenceItemFive`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemEvidenceResponseObservationsItemEvidenceItemFiveBuilder::kind)
    /// - [`problem_id`](ProblemEvidenceResponseObservationsItemEvidenceItemFiveBuilder::problem_id)
    pub fn build(self) -> Result<ProblemEvidenceResponseObservationsItemEvidenceItemFive, BuildError> {
        Ok(ProblemEvidenceResponseObservationsItemEvidenceItemFive {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
