pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ReleaseDestinationRef
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseDestinationRef {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
}

impl ReleaseDestinationRef {
    pub fn builder() -> ReleaseDestinationRefBuilder {
        <ReleaseDestinationRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseDestinationRefBuilder {
    id: Option<String>,
    slug: Option<String>,
    environment: Option<String>,
}

impl ReleaseDestinationRefBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn environment(mut self, value: impl Into<String>) -> Self {
        self.environment = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseDestinationRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ReleaseDestinationRefBuilder::id)
    /// - [`slug`](ReleaseDestinationRefBuilder::slug)
    pub fn build(self) -> Result<ReleaseDestinationRef, BuildError> {
        Ok(ReleaseDestinationRef {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            environment: self.environment,
        })
    }
}
