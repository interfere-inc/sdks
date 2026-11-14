pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProblemRelatedResponsePriorItemEvidence {
    pub kind: ProblemRelatedResponsePriorItemEvidenceKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ProblemRelatedResponsePriorItemEvidence {
    pub fn builder() -> ProblemRelatedResponsePriorItemEvidenceBuilder {
        <ProblemRelatedResponsePriorItemEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProblemRelatedResponsePriorItemEvidenceBuilder {
    kind: Option<ProblemRelatedResponsePriorItemEvidenceKind>,
    problem_id: Option<String>,
}

impl ProblemRelatedResponsePriorItemEvidenceBuilder {
    pub fn kind(mut self, value: ProblemRelatedResponsePriorItemEvidenceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProblemRelatedResponsePriorItemEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ProblemRelatedResponsePriorItemEvidenceBuilder::kind)
    /// - [`problem_id`](ProblemRelatedResponsePriorItemEvidenceBuilder::problem_id)
    pub fn build(self) -> Result<ProblemRelatedResponsePriorItemEvidence, BuildError> {
        Ok(ProblemRelatedResponsePriorItemEvidence {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
