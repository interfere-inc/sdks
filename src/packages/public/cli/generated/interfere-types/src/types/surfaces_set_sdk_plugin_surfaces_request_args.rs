pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetSdkPluginSurfacesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    pub plugin: SetSdkPluginSurfacesRequestArgsPlugin,
    #[serde(default)]
    pub enabled: bool,
}

impl SetSdkPluginSurfacesRequestArgs {
    pub fn builder() -> SetSdkPluginSurfacesRequestArgsBuilder {
        <SetSdkPluginSurfacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetSdkPluginSurfacesRequestArgsBuilder {
    surface_slug: Option<String>,
    plugin: Option<SetSdkPluginSurfacesRequestArgsPlugin>,
    enabled: Option<bool>,
}

impl SetSdkPluginSurfacesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn plugin(mut self, value: SetSdkPluginSurfacesRequestArgsPlugin) -> Self {
        self.plugin = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetSdkPluginSurfacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](SetSdkPluginSurfacesRequestArgsBuilder::surface_slug)
    /// - [`plugin`](SetSdkPluginSurfacesRequestArgsBuilder::plugin)
    /// - [`enabled`](SetSdkPluginSurfacesRequestArgsBuilder::enabled)
    pub fn build(self) -> Result<SetSdkPluginSurfacesRequestArgs, BuildError> {
        Ok(SetSdkPluginSurfacesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            plugin: self.plugin.ok_or_else(|| BuildError::missing_field("plugin"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
