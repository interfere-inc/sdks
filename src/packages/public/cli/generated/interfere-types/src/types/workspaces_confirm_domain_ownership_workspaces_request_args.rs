pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfirmDomainOwnershipWorkspacesRequestArgs {
    #[serde(rename = "clerkDomainId")]
    #[serde(default)]
    pub clerk_domain_id: String,
}

impl ConfirmDomainOwnershipWorkspacesRequestArgs {
    pub fn builder() -> ConfirmDomainOwnershipWorkspacesRequestArgsBuilder {
        <ConfirmDomainOwnershipWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfirmDomainOwnershipWorkspacesRequestArgsBuilder {
    clerk_domain_id: Option<String>,
}

impl ConfirmDomainOwnershipWorkspacesRequestArgsBuilder {
    pub fn clerk_domain_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_domain_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConfirmDomainOwnershipWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clerk_domain_id`](ConfirmDomainOwnershipWorkspacesRequestArgsBuilder::clerk_domain_id)
    pub fn build(self) -> Result<ConfirmDomainOwnershipWorkspacesRequestArgs, BuildError> {
        Ok(ConfirmDomainOwnershipWorkspacesRequestArgs {
            clerk_domain_id: self.clerk_domain_id.ok_or_else(|| BuildError::missing_field("clerk_domain_id"))?,
        })
    }
}
