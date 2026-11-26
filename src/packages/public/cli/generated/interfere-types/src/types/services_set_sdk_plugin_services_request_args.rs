pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetSdkPluginServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    pub plugin: SetSdkPluginServicesRequestArgsPlugin,
    #[serde(default)]
    pub enabled: bool,
}

impl SetSdkPluginServicesRequestArgs {
    pub fn builder() -> SetSdkPluginServicesRequestArgsBuilder {
        <SetSdkPluginServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetSdkPluginServicesRequestArgsBuilder {
    service_id: Option<String>,
    plugin: Option<SetSdkPluginServicesRequestArgsPlugin>,
    enabled: Option<bool>,
}

impl SetSdkPluginServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn plugin(mut self, value: SetSdkPluginServicesRequestArgsPlugin) -> Self {
        self.plugin = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetSdkPluginServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](SetSdkPluginServicesRequestArgsBuilder::service_id)
    /// - [`plugin`](SetSdkPluginServicesRequestArgsBuilder::plugin)
    /// - [`enabled`](SetSdkPluginServicesRequestArgsBuilder::enabled)
    pub fn build(self) -> Result<SetSdkPluginServicesRequestArgs, BuildError> {
        Ok(SetSdkPluginServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            plugin: self.plugin.ok_or_else(|| BuildError::missing_field("plugin"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}
