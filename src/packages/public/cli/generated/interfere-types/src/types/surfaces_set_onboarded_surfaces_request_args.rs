pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetOnboardedSurfacesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    #[serde(default)]
    pub onboarded: bool,
}

impl SetOnboardedSurfacesRequestArgs {
    pub fn builder() -> SetOnboardedSurfacesRequestArgsBuilder {
        <SetOnboardedSurfacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetOnboardedSurfacesRequestArgsBuilder {
    surface_slug: Option<String>,
    onboarded: Option<bool>,
}

impl SetOnboardedSurfacesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn onboarded(mut self, value: bool) -> Self {
        self.onboarded = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetOnboardedSurfacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](SetOnboardedSurfacesRequestArgsBuilder::surface_slug)
    /// - [`onboarded`](SetOnboardedSurfacesRequestArgsBuilder::onboarded)
    pub fn build(self) -> Result<SetOnboardedSurfacesRequestArgs, BuildError> {
        Ok(SetOnboardedSurfacesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            onboarded: self.onboarded.ok_or_else(|| BuildError::missing_field("onboarded"))?,
        })
    }
}
