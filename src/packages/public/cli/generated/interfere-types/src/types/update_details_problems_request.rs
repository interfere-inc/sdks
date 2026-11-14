pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDetailsProblemsRequest {
    #[serde(default)]
    pub args: UpdateDetailsProblemsRequestArgs,
}

impl UpdateDetailsProblemsRequest {
    pub fn builder() -> UpdateDetailsProblemsRequestBuilder {
        <UpdateDetailsProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDetailsProblemsRequestBuilder {
    args: Option<UpdateDetailsProblemsRequestArgs>,
}

impl UpdateDetailsProblemsRequestBuilder {
    pub fn args(mut self, value: UpdateDetailsProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDetailsProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateDetailsProblemsRequestBuilder::args)
    pub fn build(self) -> Result<UpdateDetailsProblemsRequest, BuildError> {
        Ok(UpdateDetailsProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

