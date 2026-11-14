pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetAnonymousUserTrackingSurfacesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    #[serde(default)]
    pub enabled: bool,
}

impl SetAnonymousUserTrackingSurfacesRequestArgs {
    pub fn builder() -> SetAnonymousUserTrackingSurfacesRequestArgsBuilder {
        <SetAnonymousUserTrackingSurfacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousUserTrackingSurfacesRequestArgsBuilder {
    surface_slug: Option<String>,
    enabled: Option<bool>,
}

impl SetAnonymousUserTrackingSurfacesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousUserTrackingSurfacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](SetAnonymousUserTrackingSurfacesRequestArgsBuilder::surface_slug)
    /// - [`enabled`](SetAnonymousUserTrackingSurfacesRequestArgsBuilder::enabled)
    pub fn build(self) -> Result<SetAnonymousUserTrackingSurfacesRequestArgs, BuildError> {
        Ok(SetAnonymousUserTrackingSurfacesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
