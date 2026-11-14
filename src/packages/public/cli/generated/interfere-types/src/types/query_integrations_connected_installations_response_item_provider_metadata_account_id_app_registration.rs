pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistration {
    #[serde(default)]
    pub permissions: HashMap<String, String>,
    #[serde(rename = "settingsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_url: Option<String>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistration {
    pub fn builder() -> QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistrationBuilder {
        <QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistrationBuilder {
    permissions: Option<HashMap<String, String>>,
    settings_url: Option<String>,
}

impl QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistrationBuilder {
    pub fn permissions(mut self, value: HashMap<String, String>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn settings_url(mut self, value: impl Into<String>) -> Self {
        self.settings_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistrationBuilder::permissions)
    pub fn build(self) -> Result<QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistration, BuildError> {
        Ok(QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdAppRegistration {
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            settings_url: self.settings_url,
        })
    }
}
