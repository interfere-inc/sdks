pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReadPublicKeyResponse {
    /// Publishable credential for browser SDK configuration.
    #[serde(default)]
    pub content: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
    /// Unique publishable key identifier.
    #[serde(default)]
    pub id: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    #[serde(default)]
    pub revoked: bool,
}

impl ReadPublicKeyResponse {
    pub fn builder() -> ReadPublicKeyResponseBuilder {
        <ReadPublicKeyResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReadPublicKeyResponseBuilder {
    content: Option<String>,
    name: Option<String>,
    id: Option<String>,
    surface_slug: Option<String>,
    revoked: Option<bool>,
}

impl ReadPublicKeyResponseBuilder {
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

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn revoked(mut self, value: bool) -> Self {
        self.revoked = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReadPublicKeyResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](ReadPublicKeyResponseBuilder::content)
    /// - [`name`](ReadPublicKeyResponseBuilder::name)
    /// - [`id`](ReadPublicKeyResponseBuilder::id)
    /// - [`surface_slug`](ReadPublicKeyResponseBuilder::surface_slug)
    /// - [`revoked`](ReadPublicKeyResponseBuilder::revoked)
    pub fn build(self) -> Result<ReadPublicKeyResponse, BuildError> {
        Ok(ReadPublicKeyResponse {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            revoked: self.revoked.ok_or_else(|| BuildError::missing_field("revoked"))?,
        })
    }
}
