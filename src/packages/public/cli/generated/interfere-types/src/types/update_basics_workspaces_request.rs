pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBasicsWorkspacesRequest {
    #[serde(default)]
    pub args: UpdateBasicsWorkspacesRequestArgs,
}

impl UpdateBasicsWorkspacesRequest {
    pub fn builder() -> UpdateBasicsWorkspacesRequestBuilder {
        <UpdateBasicsWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBasicsWorkspacesRequestBuilder {
    args: Option<UpdateBasicsWorkspacesRequestArgs>,
}

impl UpdateBasicsWorkspacesRequestBuilder {
    pub fn args(mut self, value: UpdateBasicsWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateBasicsWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateBasicsWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<UpdateBasicsWorkspacesRequest, BuildError> {
        Ok(UpdateBasicsWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

