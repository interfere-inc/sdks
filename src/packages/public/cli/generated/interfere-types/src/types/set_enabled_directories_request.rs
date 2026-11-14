pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetEnabledDirectoriesRequest {
    #[serde(default)]
    pub args: SetEnabledDirectoriesRequestArgs,
}

impl SetEnabledDirectoriesRequest {
    pub fn builder() -> SetEnabledDirectoriesRequestBuilder {
        <SetEnabledDirectoriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetEnabledDirectoriesRequestBuilder {
    args: Option<SetEnabledDirectoriesRequestArgs>,
}

impl SetEnabledDirectoriesRequestBuilder {
    pub fn args(mut self, value: SetEnabledDirectoriesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetEnabledDirectoriesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](SetEnabledDirectoriesRequestBuilder::args)
    pub fn build(self) -> Result<SetEnabledDirectoriesRequest, BuildError> {
        Ok(SetEnabledDirectoriesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

