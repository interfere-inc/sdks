pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateBasicsWorkspacesRequestArgs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Workspace URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
}

impl UpdateBasicsWorkspacesRequestArgs {
    pub fn builder() -> UpdateBasicsWorkspacesRequestArgsBuilder {
        <UpdateBasicsWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBasicsWorkspacesRequestArgsBuilder {
    name: Option<String>,
    slug: Option<String>,
}

impl UpdateBasicsWorkspacesRequestArgsBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateBasicsWorkspacesRequestArgs`].
    pub fn build(self) -> Result<UpdateBasicsWorkspacesRequestArgs, BuildError> {
        Ok(UpdateBasicsWorkspacesRequestArgs {
            name: self.name,
            slug: self.slug,
        })
    }
}
