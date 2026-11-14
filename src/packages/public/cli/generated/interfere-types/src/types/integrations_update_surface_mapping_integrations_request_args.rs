pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateSurfaceMappingIntegrationsRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    #[serde(rename = "sourceWorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_working_directory: Option<String>,
}

impl UpdateSurfaceMappingIntegrationsRequestArgs {
    pub fn builder() -> UpdateSurfaceMappingIntegrationsRequestArgsBuilder {
        <UpdateSurfaceMappingIntegrationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateSurfaceMappingIntegrationsRequestArgsBuilder {
    surface_slug: Option<String>,
    source_working_directory: Option<String>,
}

impl UpdateSurfaceMappingIntegrationsRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn source_working_directory(mut self, value: impl Into<String>) -> Self {
        self.source_working_directory = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateSurfaceMappingIntegrationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](UpdateSurfaceMappingIntegrationsRequestArgsBuilder::surface_slug)
    pub fn build(self) -> Result<UpdateSurfaceMappingIntegrationsRequestArgs, BuildError> {
        Ok(UpdateSurfaceMappingIntegrationsRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            source_working_directory: self.source_working_directory,
        })
    }
}
