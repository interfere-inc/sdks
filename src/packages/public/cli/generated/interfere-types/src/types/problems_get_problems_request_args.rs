pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl GetProblemsRequestArgs {
    pub fn builder() -> GetProblemsRequestArgsBuilder {
        <GetProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProblemsRequestArgsBuilder {
    problem_id: Option<String>,
}

impl GetProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GetProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](GetProblemsRequestArgsBuilder::problem_id)
    pub fn build(self) -> Result<GetProblemsRequestArgs, BuildError> {
        Ok(GetProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
