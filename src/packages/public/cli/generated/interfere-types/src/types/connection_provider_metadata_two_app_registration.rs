pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionProviderMetadataTwoAppRegistration {
    #[serde(default)]
    pub permissions: HashMap<String, String>,
    #[serde(rename = "settingsUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_url: Option<String>,
}

impl ConnectionProviderMetadataTwoAppRegistration {
    pub fn builder() -> ConnectionProviderMetadataTwoAppRegistrationBuilder {
        <ConnectionProviderMetadataTwoAppRegistrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionProviderMetadataTwoAppRegistrationBuilder {
    permissions: Option<HashMap<String, String>>,
    settings_url: Option<String>,
}

impl ConnectionProviderMetadataTwoAppRegistrationBuilder {
    pub fn permissions(mut self, value: HashMap<String, String>) -> Self {
        self.permissions = Some(value);
        self
    }

    pub fn settings_url(mut self, value: impl Into<String>) -> Self {
        self.settings_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionProviderMetadataTwoAppRegistration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`permissions`](ConnectionProviderMetadataTwoAppRegistrationBuilder::permissions)
    pub fn build(self) -> Result<ConnectionProviderMetadataTwoAppRegistration, BuildError> {
        Ok(ConnectionProviderMetadataTwoAppRegistration {
            permissions: self.permissions.ok_or_else(|| BuildError::missing_field("permissions"))?,
            settings_url: self.settings_url,
        })
    }
}
