pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RemoveProxyDomainWorkspacesRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default)]
    pub name: String,
}

impl RemoveProxyDomainWorkspacesRequestArgs {
    pub fn builder() -> RemoveProxyDomainWorkspacesRequestArgsBuilder {
        <RemoveProxyDomainWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RemoveProxyDomainWorkspacesRequestArgsBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl RemoveProxyDomainWorkspacesRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RemoveProxyDomainWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](RemoveProxyDomainWorkspacesRequestArgsBuilder::name)
    pub fn build(self) -> Result<RemoveProxyDomainWorkspacesRequestArgs, BuildError> {
        Ok(RemoveProxyDomainWorkspacesRequestArgs {
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
