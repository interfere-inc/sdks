pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkProblemDuplicateResponse {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
    #[serde(rename = "keeperProblemId")]
    #[serde(default)]
    pub keeper_problem_id: String,
}

impl MarkProblemDuplicateResponse {
    pub fn builder() -> MarkProblemDuplicateResponseBuilder {
        <MarkProblemDuplicateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkProblemDuplicateResponseBuilder {
    problem_id: Option<String>,
    keeper_problem_id: Option<String>,
}

impl MarkProblemDuplicateResponseBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    pub fn keeper_problem_id(mut self, value: impl Into<String>) -> Self {
        self.keeper_problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MarkProblemDuplicateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](MarkProblemDuplicateResponseBuilder::problem_id)
    /// - [`keeper_problem_id`](MarkProblemDuplicateResponseBuilder::keeper_problem_id)
    pub fn build(self) -> Result<MarkProblemDuplicateResponse, BuildError> {
        Ok(MarkProblemDuplicateResponse {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
            keeper_problem_id: self.keeper_problem_id.ok_or_else(|| BuildError::missing_field("keeper_problem_id"))?,
        })
    }
}
