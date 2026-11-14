pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePriorityProblemsRequest {
    #[serde(default)]
    pub args: UpdatePriorityProblemsRequestArgs,
}

impl UpdatePriorityProblemsRequest {
    pub fn builder() -> UpdatePriorityProblemsRequestBuilder {
        <UpdatePriorityProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePriorityProblemsRequestBuilder {
    args: Option<UpdatePriorityProblemsRequestArgs>,
}

impl UpdatePriorityProblemsRequestBuilder {
    pub fn args(mut self, value: UpdatePriorityProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePriorityProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdatePriorityProblemsRequestBuilder::args)
    pub fn build(self) -> Result<UpdatePriorityProblemsRequest, BuildError> {
        Ok(UpdatePriorityProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

