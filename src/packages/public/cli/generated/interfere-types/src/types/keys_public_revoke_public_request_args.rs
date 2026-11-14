pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RevokePublicRequestArgs {
    #[serde(default)]
    pub id: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl RevokePublicRequestArgs {
    pub fn builder() -> RevokePublicRequestArgsBuilder {
        <RevokePublicRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RevokePublicRequestArgsBuilder {
    id: Option<String>,
    surface_slug: Option<String>,
}

impl RevokePublicRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RevokePublicRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RevokePublicRequestArgsBuilder::id)
    /// - [`surface_slug`](RevokePublicRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<RevokePublicRequestArgs, BuildError> {
        Ok(RevokePublicRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
