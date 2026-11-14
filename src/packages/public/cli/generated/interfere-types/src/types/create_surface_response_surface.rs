pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateSurfaceResponseSurface {
    /// Unique surface identifier.
    #[serde(default)]
    pub id: String,
    /// Display name of the surface.
    #[serde(default)]
    pub name: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
    /// Application framework used by the surface.
    pub r#type: CreateSurfaceResponseSurfaceType,
}

impl CreateSurfaceResponseSurface {
    pub fn builder() -> CreateSurfaceResponseSurfaceBuilder {
        <CreateSurfaceResponseSurfaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSurfaceResponseSurfaceBuilder {
    id: Option<String>,
    name: Option<String>,
    slug: Option<String>,
    r#type: Option<CreateSurfaceResponseSurfaceType>,
}

impl CreateSurfaceResponseSurfaceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateSurfaceResponseSurfaceType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSurfaceResponseSurface`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSurfaceResponseSurfaceBuilder::id)
    /// - [`name`](CreateSurfaceResponseSurfaceBuilder::name)
    /// - [`slug`](CreateSurfaceResponseSurfaceBuilder::slug)
    /// - [`r#type`](CreateSurfaceResponseSurfaceBuilder::r#type)
    pub fn build(self) -> Result<CreateSurfaceResponseSurface, BuildError> {
        Ok(CreateSurfaceResponseSurface {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
