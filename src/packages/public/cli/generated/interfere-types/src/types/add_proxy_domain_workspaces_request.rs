pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddProxyDomainWorkspacesRequest {
    #[serde(default)]
    pub args: AddProxyDomainWorkspacesRequestArgs,
}

impl AddProxyDomainWorkspacesRequest {
    pub fn builder() -> AddProxyDomainWorkspacesRequestBuilder {
        <AddProxyDomainWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddProxyDomainWorkspacesRequestBuilder {
    args: Option<AddProxyDomainWorkspacesRequestArgs>,
}

impl AddProxyDomainWorkspacesRequestBuilder {
    pub fn args(mut self, value: AddProxyDomainWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddProxyDomainWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](AddProxyDomainWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<AddProxyDomainWorkspacesRequest, BuildError> {
        Ok(AddProxyDomainWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

