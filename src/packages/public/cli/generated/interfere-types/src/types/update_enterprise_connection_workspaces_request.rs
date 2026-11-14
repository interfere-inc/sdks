pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEnterpriseConnectionWorkspacesRequest {
    #[serde(default)]
    pub args: UpdateEnterpriseConnectionWorkspacesRequestArgs,
}

impl UpdateEnterpriseConnectionWorkspacesRequest {
    pub fn builder() -> UpdateEnterpriseConnectionWorkspacesRequestBuilder {
        <UpdateEnterpriseConnectionWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEnterpriseConnectionWorkspacesRequestBuilder {
    args: Option<UpdateEnterpriseConnectionWorkspacesRequestArgs>,
}

impl UpdateEnterpriseConnectionWorkspacesRequestBuilder {
    pub fn args(mut self, value: UpdateEnterpriseConnectionWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEnterpriseConnectionWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](UpdateEnterpriseConnectionWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<UpdateEnterpriseConnectionWorkspacesRequest, BuildError> {
        Ok(UpdateEnterpriseConnectionWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

