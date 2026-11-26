pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForWorkspaceServicesRequest {
    #[serde(default)]
    pub args: ForWorkspaceServicesRequestArgs,
}

impl ForWorkspaceServicesRequest {
    pub fn builder() -> ForWorkspaceServicesRequestBuilder {
        <ForWorkspaceServicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForWorkspaceServicesRequestBuilder {
    args: Option<ForWorkspaceServicesRequestArgs>,
}

impl ForWorkspaceServicesRequestBuilder {
    pub fn args(mut self, value: ForWorkspaceServicesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForWorkspaceServicesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ForWorkspaceServicesRequestBuilder::args)
    pub fn build(self) -> Result<ForWorkspaceServicesRequest, BuildError> {
        Ok(ForWorkspaceServicesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

