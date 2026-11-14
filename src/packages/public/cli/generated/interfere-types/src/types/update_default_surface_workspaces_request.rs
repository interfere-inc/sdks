pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDefaultSurfaceWorkspacesRequest {
    #[serde(default)]
    pub args: UpdateDefaultSurfaceWorkspacesRequestArgs,
}

impl UpdateDefaultSurfaceWorkspacesRequest {
    pub fn builder() -> UpdateDefaultSurfaceWorkspacesRequestBuilder {
        <UpdateDefaultSurfaceWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDefaultSurfaceWorkspacesRequestBuilder {
    args: Option<UpdateDefaultSurfaceWorkspacesRequestArgs>,
}

impl UpdateDefaultSurfaceWorkspacesRequestBuilder {
    pub fn args(mut self, value: UpdateDefaultSurfaceWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDefaultSurfaceWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateDefaultSurfaceWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<UpdateDefaultSurfaceWorkspacesRequest, BuildError> {
        Ok(UpdateDefaultSurfaceWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

