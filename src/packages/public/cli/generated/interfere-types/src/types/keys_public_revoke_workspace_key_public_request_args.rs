pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokeWorkspaceKeyPublicRequestArgs {
    #[serde(default)]
    pub id: String,
}

impl RevokeWorkspaceKeyPublicRequestArgs {
    pub fn builder() -> RevokeWorkspaceKeyPublicRequestArgsBuilder {
        <RevokeWorkspaceKeyPublicRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokeWorkspaceKeyPublicRequestArgsBuilder {
    id: Option<String>,
}

impl RevokeWorkspaceKeyPublicRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevokeWorkspaceKeyPublicRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RevokeWorkspaceKeyPublicRequestArgsBuilder::id)
    pub fn build(self) -> Result<RevokeWorkspaceKeyPublicRequestArgs, BuildError> {
        Ok(RevokeWorkspaceKeyPublicRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
