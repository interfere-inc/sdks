pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EditablePropertiesProblemsRequestArgs {
    #[serde(rename = "problemId")]
    #[serde(default)]
    pub problem_id: String,
}

impl EditablePropertiesProblemsRequestArgs {
    pub fn builder() -> EditablePropertiesProblemsRequestArgsBuilder {
        <EditablePropertiesProblemsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EditablePropertiesProblemsRequestArgsBuilder {
    problem_id: Option<String>,
}

impl EditablePropertiesProblemsRequestArgsBuilder {
    pub fn problem_id(mut self, value: impl Into<String>) -> Self {
        self.problem_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EditablePropertiesProblemsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`problem_id`](EditablePropertiesProblemsRequestArgsBuilder::problem_id)
    pub fn build(self) -> Result<EditablePropertiesProblemsRequestArgs, BuildError> {
        Ok(EditablePropertiesProblemsRequestArgs {
            problem_id: self.problem_id.ok_or_else(|| BuildError::missing_field("problem_id"))?,
        })
    }
}
