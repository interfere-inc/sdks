pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadWorkspacePublicKeyResponse {
    /// Publishable credential for browser SDK configuration.
    #[serde(default)]
    pub content: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
    /// Unique publishable key identifier.
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub revoked: bool,
}

impl ReadWorkspacePublicKeyResponse {
    pub fn builder() -> ReadWorkspacePublicKeyResponseBuilder {
        <ReadWorkspacePublicKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadWorkspacePublicKeyResponseBuilder {
    content: Option<String>,
    name: Option<String>,
    id: Option<String>,
    revoked: Option<bool>,
}

impl ReadWorkspacePublicKeyResponseBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadWorkspacePublicKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ReadWorkspacePublicKeyResponseBuilder::content)
    /// - [`name`](ReadWorkspacePublicKeyResponseBuilder::name)
    /// - [`id`](ReadWorkspacePublicKeyResponseBuilder::id)
    /// - [`revoked`](ReadWorkspacePublicKeyResponseBuilder::revoked)
    pub fn build(self) -> Result<ReadWorkspacePublicKeyResponse, BuildError> {
        Ok(ReadWorkspacePublicKeyResponse {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            revoked: self.revoked.ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
