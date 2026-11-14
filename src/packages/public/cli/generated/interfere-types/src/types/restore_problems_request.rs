pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RestoreProblemsRequest {
    #[serde(default)]
    pub args: RestoreProblemsRequestArgs,
}

impl RestoreProblemsRequest {
    pub fn builder() -> RestoreProblemsRequestBuilder {
        <RestoreProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RestoreProblemsRequestBuilder {
    args: Option<RestoreProblemsRequestArgs>,
}

impl RestoreProblemsRequestBuilder {
    pub fn args(mut self, value: RestoreProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RestoreProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RestoreProblemsRequestBuilder::args)
    pub fn build(self) -> Result<RestoreProblemsRequest, BuildError> {
        Ok(RestoreProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

