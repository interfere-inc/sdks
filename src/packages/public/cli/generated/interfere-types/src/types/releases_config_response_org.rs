pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleasesConfigResponseOrg {
    #[serde(default)]
    pub id: String,
    /// Workspace URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
}

impl ReleasesConfigResponseOrg {
    pub fn builder() -> ReleasesConfigResponseOrgBuilder {
        <ReleasesConfigResponseOrgBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleasesConfigResponseOrgBuilder {
    id: Option<String>,
    slug: Option<String>,
}

impl ReleasesConfigResponseOrgBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleasesConfigResponseOrg`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReleasesConfigResponseOrgBuilder::id)
    /// - [`slug`](ReleasesConfigResponseOrgBuilder::slug)
    pub fn build(self) -> Result<ReleasesConfigResponseOrg, BuildError> {
        Ok(ReleasesConfigResponseOrg {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
