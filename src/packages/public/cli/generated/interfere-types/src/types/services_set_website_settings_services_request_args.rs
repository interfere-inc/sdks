pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SetWebsiteSettingsServicesRequestArgs {
    #[serde(rename = "serviceId")]
    #[serde(default)]
    pub service_id: String,
    #[serde(rename = "websiteSettings")]
    pub website_settings: SetWebsiteSettingsServicesRequestArgsWebsiteSettings,
}

impl SetWebsiteSettingsServicesRequestArgs {
    pub fn builder() -> SetWebsiteSettingsServicesRequestArgsBuilder {
        <SetWebsiteSettingsServicesRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetWebsiteSettingsServicesRequestArgsBuilder {
    service_id: Option<String>,
    website_settings: Option<SetWebsiteSettingsServicesRequestArgsWebsiteSettings>,
}

impl SetWebsiteSettingsServicesRequestArgsBuilder {
    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn website_settings(mut self, value: SetWebsiteSettingsServicesRequestArgsWebsiteSettings) -> Self {
        self.website_settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetWebsiteSettingsServicesRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_id`](SetWebsiteSettingsServicesRequestArgsBuilder::service_id)
    /// - [`website_settings`](SetWebsiteSettingsServicesRequestArgsBuilder::website_settings)
    pub fn build(self) -> Result<SetWebsiteSettingsServicesRequestArgs, BuildError> {
        Ok(SetWebsiteSettingsServicesRequestArgs {
            service_id: self.service_id.ok_or_else(|| BuildError::missing_field("service_id"))?,
            website_settings: self.website_settings.ok_or_else(|| BuildError::missing_field("website_settings"))?,
        })
    }
}
