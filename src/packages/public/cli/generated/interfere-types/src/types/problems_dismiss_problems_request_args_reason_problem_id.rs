pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DismissProblemsRequestArgsReasonProblemId {
    pub kind: DismissProblemsRequestArgsReasonProblemIdKind,
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl DismissProblemsRequestArgsReasonProblemId {
    pub fn builder() -> DismissProblemsRequestArgsReasonProblemIdBuilder {
        <DismissProblemsRequestArgsReasonProblemIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DismissProblemsRequestArgsReasonProblemIdBuilder {
    kind: Option<DismissProblemsRequestArgsReasonProblemIdKind>,
    problem_id: Option<String>,
}

impl DismissProblemsRequestArgsReasonProblemIdBuilder {
    pub fn kind(mut self, value: DismissProblemsRequestArgsReasonProblemIdKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DismissProblemsRequestArgsReasonProblemId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](DismissProblemsRequestArgsReasonProblemIdBuilder::kind)
    /// - [`problem_id`](DismissProblemsRequestArgsReasonProblemIdBuilder::problem_id)
    pub fn build(self) -> Result<DismissProblemsRequestArgsReasonProblemId, BuildError> {
        Ok(DismissProblemsRequestArgsReasonProblemId {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
