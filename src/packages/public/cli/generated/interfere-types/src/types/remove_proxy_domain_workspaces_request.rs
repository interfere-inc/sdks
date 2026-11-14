pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveProxyDomainWorkspacesRequest {
    #[serde(default)]
    pub args: RemoveProxyDomainWorkspacesRequestArgs,
}

impl RemoveProxyDomainWorkspacesRequest {
    pub fn builder() -> RemoveProxyDomainWorkspacesRequestBuilder {
        <RemoveProxyDomainWorkspacesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveProxyDomainWorkspacesRequestBuilder {
    args: Option<RemoveProxyDomainWorkspacesRequestArgs>,
}

impl RemoveProxyDomainWorkspacesRequestBuilder {
    pub fn args(mut self, value: RemoveProxyDomainWorkspacesRequestArgs) -> Self {
        self.args = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RemoveProxyDomainWorkspacesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`args`](RemoveProxyDomainWorkspacesRequestBuilder::args)
    pub fn build(self) -> Result<RemoveProxyDomainWorkspacesRequest, BuildError> {
        Ok(RemoveProxyDomainWorkspacesRequest {
            args: self.args.ok_or_else(|| BuildError::missing_field("args"))?,
        })
    }
}

