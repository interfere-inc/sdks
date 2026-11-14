pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WorkspaceCreateResponse {
    #[serde(default)]
    pub id: String,
    /// Workspace URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(default)]
    pub slug: String,
}

impl WorkspaceCreateResponse {
    pub fn builder() -> WorkspaceCreateResponseBuilder {
        <WorkspaceCreateResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WorkspaceCreateResponseBuilder {
    id: Option<String>,
    slug: Option<String>,
}

impl WorkspaceCreateResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WorkspaceCreateResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WorkspaceCreateResponseBuilder::id)
    /// - [`slug`](WorkspaceCreateResponseBuilder::slug)
    pub fn build(self) -> Result<WorkspaceCreateResponse, BuildError> {
        Ok(WorkspaceCreateResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
