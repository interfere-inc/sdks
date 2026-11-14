pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviousAccountsProblemsRequest {
    #[serde(default)]
    pub args: PreviousAccountsProblemsRequestArgs,
}

impl PreviousAccountsProblemsRequest {
    pub fn builder() -> PreviousAccountsProblemsRequestBuilder {
        <PreviousAccountsProblemsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviousAccountsProblemsRequestBuilder {
    args: Option<PreviousAccountsProblemsRequestArgs>,
}

impl PreviousAccountsProblemsRequestBuilder {
    pub fn args(mut self, value: PreviousAccountsProblemsRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviousAccountsProblemsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](PreviousAccountsProblemsRequestBuilder::args)
    pub fn build(self) -> Result<PreviousAccountsProblemsRequest, BuildError> {
        Ok(PreviousAccountsProblemsRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

