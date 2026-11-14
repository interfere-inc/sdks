pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// OrgRef
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrgRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub slug: String,
}

impl OrgRef {
    pub fn builder() -> OrgRefBuilder {
        <OrgRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrgRefBuilder {
    id: Option<String>,
    slug: Option<String>,
}

impl OrgRefBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OrgRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OrgRefBuilder::id)
    /// - [`slug`](OrgRefBuilder::slug)
    pub fn build(self) -> Result<OrgRef, BuildError> {
        Ok(OrgRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
