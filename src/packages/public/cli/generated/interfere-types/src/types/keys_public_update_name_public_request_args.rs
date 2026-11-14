pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNamePublicRequestArgs {
    #[serde(default)]
    pub id: String,
    /// Display name of the key.
    #[serde(default)]
    pub name: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl UpdateNamePublicRequestArgs {
    pub fn builder() -> UpdateNamePublicRequestArgsBuilder {
        <UpdateNamePublicRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNamePublicRequestArgsBuilder {
    id: Option<String>,
    name: Option<String>,
    surface_slug: Option<String>,
}

impl UpdateNamePublicRequestArgsBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateNamePublicRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateNamePublicRequestArgsBuilder::id)
    /// - [`name`](UpdateNamePublicRequestArgsBuilder::name)
    /// - [`surface_slug`](UpdateNamePublicRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<UpdateNamePublicRequestArgs, BuildError> {
        Ok(UpdateNamePublicRequestArgs {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
