pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesRemoveProxyDomainResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesRemoveProxyDomainResponse {
    pub fn builder() -> MutationWorkspacesRemoveProxyDomainResponseBuilder {
        <MutationWorkspacesRemoveProxyDomainResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesRemoveProxyDomainResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesRemoveProxyDomainResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesRemoveProxyDomainResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesRemoveProxyDomainResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesRemoveProxyDomainResponse, BuildError> {
        Ok(MutationWorkspacesRemoveProxyDomainResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
