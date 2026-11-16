pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfirmDomainOwnershipWorkspacesRequest {
    #[serde(default)]
    pub args: ConfirmDomainOwnershipWorkspacesRequestArgs,
}

impl ConfirmDomainOwnershipWorkspacesRequest {
    pub fn builder() -> ConfirmDomainOwnershipWorkspacesRequestBuilder {
        <ConfirmDomainOwnershipWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmDomainOwnershipWorkspacesRequestBuilder {
    args: Option<ConfirmDomainOwnershipWorkspacesRequestArgs>,
}

impl ConfirmDomainOwnershipWorkspacesRequestBuilder {
    pub fn args(mut self, value: ConfirmDomainOwnershipWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfirmDomainOwnershipWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](ConfirmDomainOwnershipWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<ConfirmDomainOwnershipWorkspacesRequest, BuildError> {
        Ok(ConfirmDomainOwnershipWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

