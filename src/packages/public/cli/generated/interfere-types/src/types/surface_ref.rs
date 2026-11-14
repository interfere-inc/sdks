pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// SurfaceRef
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SurfaceRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub slug: String,
}

impl SurfaceRef {
    pub fn builder() -> SurfaceRefBuilder {
        <SurfaceRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SurfaceRefBuilder {
    id: Option<String>,
    slug: Option<String>,
}

impl SurfaceRefBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SurfaceRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SurfaceRefBuilder::id)
    /// - [`slug`](SurfaceRefBuilder::slug)
    pub fn build(self) -> Result<SurfaceRef, BuildError> {
        Ok(SurfaceRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
