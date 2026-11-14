pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetEnterpriseConnectionActiveWorkspacesRequestArgs {
    #[serde(default)]
    pub active: bool,
    #[serde(default)]
    pub id: String,
}

impl SetEnterpriseConnectionActiveWorkspacesRequestArgs {
    pub fn builder() -> SetEnterpriseConnectionActiveWorkspacesRequestArgsBuilder {
        <SetEnterpriseConnectionActiveWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetEnterpriseConnectionActiveWorkspacesRequestArgsBuilder {
    active: Option<bool>,
    id: Option<String>,
}

impl SetEnterpriseConnectionActiveWorkspacesRequestArgsBuilder {
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetEnterpriseConnectionActiveWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`active`](SetEnterpriseConnectionActiveWorkspacesRequestArgsBuilder::active)
    /// - [`id`](SetEnterpriseConnectionActiveWorkspacesRequestArgsBuilder::id)
    pub fn build(self) -> Result<SetEnterpriseConnectionActiveWorkspacesRequestArgs, BuildError> {
        Ok(SetEnterpriseConnectionActiveWorkspacesRequestArgs {
            active: self.active.ok_or_else(|| BuildError::missing_field("active"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
