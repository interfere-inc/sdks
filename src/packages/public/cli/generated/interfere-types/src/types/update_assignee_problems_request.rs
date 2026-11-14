pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAssigneeProblemsRequest {
    #[serde(default)]
    pub args: UpdateAssigneeProblemsRequestArgs,
}

impl UpdateAssigneeProblemsRequest {
    pub fn builder() -> UpdateAssigneeProblemsRequestBuilder {
        <UpdateAssigneeProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAssigneeProblemsRequestBuilder {
    args: Option<UpdateAssigneeProblemsRequestArgs>,
}

impl UpdateAssigneeProblemsRequestBuilder {
    pub fn args(mut self, value: UpdateAssigneeProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAssigneeProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateAssigneeProblemsRequestBuilder::args)
    pub fn build(self) -> Result<UpdateAssigneeProblemsRequest, BuildError> {
        Ok(UpdateAssigneeProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

