pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateNameSurfacesRequestArgs {
    #[serde(default)]
    pub name: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
}

impl UpdateNameSurfacesRequestArgs {
    pub fn builder() -> UpdateNameSurfacesRequestArgsBuilder {
        <UpdateNameSurfacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateNameSurfacesRequestArgsBuilder {
    name: Option<String>,
    surface_slug: Option<String>,
}

impl UpdateNameSurfacesRequestArgsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateNameSurfacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](UpdateNameSurfacesRequestArgsBuilder::name)
    /// - [`surface_slug`](UpdateNameSurfacesRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<UpdateNameSurfacesRequestArgs, BuildError> {
        Ok(UpdateNameSurfacesRequestArgs {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
        })
    }
}
