pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetProblemsRequest {
    #[serde(default)]
    pub args: GetProblemsRequestArgs,
}

impl GetProblemsRequest {
    pub fn builder() -> GetProblemsRequestBuilder {
        <GetProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetProblemsRequestBuilder {
    args: Option<GetProblemsRequestArgs>,
}

impl GetProblemsRequestBuilder {
    pub fn args(mut self, value: GetProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](GetProblemsRequestBuilder::args)
    pub fn build(self) -> Result<GetProblemsRequest, BuildError> {
        Ok(GetProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

