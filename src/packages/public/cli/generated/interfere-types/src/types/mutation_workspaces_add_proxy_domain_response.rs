pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MutationWorkspacesAddProxyDomainResponse {
    #[serde(default)]
    pub success: bool,
}

impl MutationWorkspacesAddProxyDomainResponse {
    pub fn builder() -> MutationWorkspacesAddProxyDomainResponseBuilder {
        <MutationWorkspacesAddProxyDomainResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MutationWorkspacesAddProxyDomainResponseBuilder {
    success: Option<bool>,
}

impl MutationWorkspacesAddProxyDomainResponseBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MutationWorkspacesAddProxyDomainResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](MutationWorkspacesAddProxyDomainResponseBuilder::success)
    pub fn build(self) -> Result<MutationWorkspacesAddProxyDomainResponse, BuildError> {
        Ok(MutationWorkspacesAddProxyDomainResponse {
            success: self.success.ok_or_else(|| BuildError::missing_field("success"))?,
        })
    }
}
