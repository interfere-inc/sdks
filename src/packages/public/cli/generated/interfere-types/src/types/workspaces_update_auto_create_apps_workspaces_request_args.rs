pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAutoCreateAppsWorkspacesRequestArgs {
    #[serde(rename = "autoCreateApps")]
    #[serde(default)]
    pub auto_create_apps: bool,
}

impl UpdateAutoCreateAppsWorkspacesRequestArgs {
    pub fn builder() -> UpdateAutoCreateAppsWorkspacesRequestArgsBuilder {
        <UpdateAutoCreateAppsWorkspacesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAutoCreateAppsWorkspacesRequestArgsBuilder {
    auto_create_apps: Option<bool>,
}

impl UpdateAutoCreateAppsWorkspacesRequestArgsBuilder {
    pub fn auto_create_apps(mut self, value: bool) -> Self {
        self.auto_create_apps = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAutoCreateAppsWorkspacesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auto_create_apps`](UpdateAutoCreateAppsWorkspacesRequestArgsBuilder::auto_create_apps)
    pub fn build(self) -> Result<UpdateAutoCreateAppsWorkspacesRequestArgs, BuildError> {
        Ok(UpdateAutoCreateAppsWorkspacesRequestArgs {
            auto_create_apps: self.auto_create_apps.ok_or_else(|| BuildError::missing_field("auto_create_apps"))?,
        })
    }
}
