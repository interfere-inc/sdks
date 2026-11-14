pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemRelatedResponseSameRequestItemEvidence {
    pub kind: ProblemRelatedResponseSameRequestItemEvidenceKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ProblemRelatedResponseSameRequestItemEvidence {
    pub fn builder() -> ProblemRelatedResponseSameRequestItemEvidenceBuilder {
        <ProblemRelatedResponseSameRequestItemEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemRelatedResponseSameRequestItemEvidenceBuilder {
    kind: Option<ProblemRelatedResponseSameRequestItemEvidenceKind>,
    problem_id: Option<String>,
}

impl ProblemRelatedResponseSameRequestItemEvidenceBuilder {
    pub fn kind(mut self, value: ProblemRelatedResponseSameRequestItemEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemRelatedResponseSameRequestItemEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemRelatedResponseSameRequestItemEvidenceBuilder::kind)
    /// - [`problem_id`](ProblemRelatedResponseSameRequestItemEvidenceBuilder::problem_id)
    pub fn build(self) -> Result<ProblemRelatedResponseSameRequestItemEvidence, BuildError> {
        Ok(ProblemRelatedResponseSameRequestItemEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
