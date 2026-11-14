pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAutoCreateAppsWorkspacesRequest {
    #[serde(default)]
    pub args: UpdateAutoCreateAppsWorkspacesRequestArgs,
}

impl UpdateAutoCreateAppsWorkspacesRequest {
    pub fn builder() -> UpdateAutoCreateAppsWorkspacesRequestBuilder {
        <UpdateAutoCreateAppsWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAutoCreateAppsWorkspacesRequestBuilder {
    args: Option<UpdateAutoCreateAppsWorkspacesRequestArgs>,
}

impl UpdateAutoCreateAppsWorkspacesRequestBuilder {
    pub fn args(mut self, value: UpdateAutoCreateAppsWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAutoCreateAppsWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateAutoCreateAppsWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<UpdateAutoCreateAppsWorkspacesRequest, BuildError> {
        Ok(UpdateAutoCreateAppsWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

