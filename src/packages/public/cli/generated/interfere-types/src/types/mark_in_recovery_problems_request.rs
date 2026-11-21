pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MarkInRecoveryProblemsRequest {
    #[serde(default)]
    pub args: MarkInRecoveryProblemsRequestArgs,
}

impl MarkInRecoveryProblemsRequest {
    pub fn builder() -> MarkInRecoveryProblemsRequestBuilder {
        <MarkInRecoveryProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MarkInRecoveryProblemsRequestBuilder {
    args: Option<MarkInRecoveryProblemsRequestArgs>,
}

impl MarkInRecoveryProblemsRequestBuilder {
    pub fn args(mut self, value: MarkInRecoveryProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MarkInRecoveryProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](MarkInRecoveryProblemsRequestBuilder::args)
    pub fn build(self) -> Result<MarkInRecoveryProblemsRequest, BuildError> {
        Ok(MarkInRecoveryProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

