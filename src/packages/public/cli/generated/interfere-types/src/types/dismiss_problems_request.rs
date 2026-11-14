pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DismissProblemsRequest {
    #[serde(default)]
    pub args: DismissProblemsRequestArgs,
}

impl DismissProblemsRequest {
    pub fn builder() -> DismissProblemsRequestBuilder {
        <DismissProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DismissProblemsRequestBuilder {
    args: Option<DismissProblemsRequestArgs>,
}

impl DismissProblemsRequestBuilder {
    pub fn args(mut self, value: DismissProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DismissProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](DismissProblemsRequestBuilder::args)
    pub fn build(self) -> Result<DismissProblemsRequest, BuildError> {
        Ok(DismissProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

