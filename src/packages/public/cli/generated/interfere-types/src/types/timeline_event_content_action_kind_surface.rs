pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentActionKindSurface {
    #[serde(default)]
    pub name: String,
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
}

impl TimelineEventContentActionKindSurface {
    pub fn builder() -> TimelineEventContentActionKindSurfaceBuilder {
        <TimelineEventContentActionKindSurfaceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentActionKindSurfaceBuilder {
    name: Option<String>,
    slug: Option<String>,
}

impl TimelineEventContentActionKindSurfaceBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentActionKindSurface`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](TimelineEventContentActionKindSurfaceBuilder::name)
    /// - [`slug`](TimelineEventContentActionKindSurfaceBuilder::slug)
    pub fn build(self) -> Result<TimelineEventContentActionKindSurface, BuildError> {
        Ok(TimelineEventContentActionKindSurface {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
