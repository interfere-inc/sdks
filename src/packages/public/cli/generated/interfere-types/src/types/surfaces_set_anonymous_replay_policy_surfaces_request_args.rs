pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetAnonymousReplayPolicySurfacesRequestArgs {
    /// Surface URL slug, using lowercase letters, numbers, and hyphens.
    #[serde(rename = "surfaceSlug")]
    #[serde(default)]
    pub surface_slug: String,
    /// When sessions without an identified user record replay: always, only once an error occurs, or never.
    pub policy: SetAnonymousReplayPolicySurfacesRequestArgsPolicy,
}

impl SetAnonymousReplayPolicySurfacesRequestArgs {
    pub fn builder() -> SetAnonymousReplayPolicySurfacesRequestArgsBuilder {
        <SetAnonymousReplayPolicySurfacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetAnonymousReplayPolicySurfacesRequestArgsBuilder {
    surface_slug: Option<String>,
    policy: Option<SetAnonymousReplayPolicySurfacesRequestArgsPolicy>,
}

impl SetAnonymousReplayPolicySurfacesRequestArgsBuilder {
    pub fn surface_slug(mut self, value: impl Into<String>) -> Self {
        self.surface_slug = Some(value.into());
        self
    }

    pub fn policy(mut self, value: SetAnonymousReplayPolicySurfacesRequestArgsPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetAnonymousReplayPolicySurfacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`surface_slug`](SetAnonymousReplayPolicySurfacesRequestArgsBuilder::surface_slug)
    /// - [`policy`](SetAnonymousReplayPolicySurfacesRequestArgsBuilder::policy)
    pub fn build(self) -> Result<SetAnonymousReplayPolicySurfacesRequestArgs, BuildError> {
        Ok(SetAnonymousReplayPolicySurfacesRequestArgs {
            surface_slug: self.surface_slug.ok_or_else(|| BuildError::missing_field("surface_slug"))?,
            policy: self.policy.ok_or_else(|| BuildError::missing_field("policy"))?,
        })
    }
}
