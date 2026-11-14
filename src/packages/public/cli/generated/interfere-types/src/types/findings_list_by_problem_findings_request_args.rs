pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListByProblemFindingsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl ListByProblemFindingsRequestArgs {
    pub fn builder() -> ListByProblemFindingsRequestArgsBuilder {
        <ListByProblemFindingsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListByProblemFindingsRequestArgsBuilder {
    problem_id: Option<String>,
}

impl ListByProblemFindingsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListByProblemFindingsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](ListByProblemFindingsRequestArgsBuilder::problem_id)
    pub fn build(self) -> Result<ListByProblemFindingsRequestArgs, BuildError> {
        Ok(ListByProblemFindingsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
