pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistration {
    #[serde(default)]
    pub permissions: HashMap<String, String>,
    #[serde(rename = "settingsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_url: Option<String>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistration {
    pub fn builder() -> QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistrationBuilder {
        <QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistrationBuilder {
    permissions: Option<HashMap<String, String>>,
    settings_url: Option<String>,
}

impl QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistrationBuilder {
    pub fn permissions(mut self, value: HashMap<String, String>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn settings_url(mut self, value: impl Into<String>) -> Self {
        self.settings_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistrationBuilder::permissions)
    pub fn build(self) -> Result<QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistration, BuildError> {
        Ok(QueryIntegrationsCloudflareInstallationResponseProviderMetadataTwoAppRegistration {
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            settings_url: self.settings_url,
        })
    }
}
