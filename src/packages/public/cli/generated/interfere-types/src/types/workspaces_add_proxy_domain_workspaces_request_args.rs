pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddProxyDomainWorkspacesRequestArgs {
    /// Unique tracking domain identifier.
    #[serde(default)]
    pub id: String,
    /// Lowercase fully qualified tracking hostname, without a trailing dot.
    #[serde(default)]
    pub name: String,
}

impl AddProxyDomainWorkspacesRequestArgs {
    pub fn builder() -> AddProxyDomainWorkspacesRequestArgsBuilder {
        <AddProxyDomainWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddProxyDomainWorkspacesRequestArgsBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl AddProxyDomainWorkspacesRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddProxyDomainWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AddProxyDomainWorkspacesRequestArgsBuilder::id)
    /// - [`name`](AddProxyDomainWorkspacesRequestArgsBuilder::name)
    pub fn build(self) -> Result<AddProxyDomainWorkspacesRequestArgs, BuildError> {
        Ok(AddProxyDomainWorkspacesRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
