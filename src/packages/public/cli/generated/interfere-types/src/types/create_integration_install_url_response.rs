pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum CreateIntegrationInstallUrlResponse {
        CreateIntegrationInstallUrlResponseInstallationUrl(CreateIntegrationInstallUrlResponseInstallationUrl),

        CreateIntegrationInstallUrlResponseRegisterUrl(CreateIntegrationInstallUrlResponseRegisterUrl),
}

impl CreateIntegrationInstallUrlResponse {
    pub fn is_create_integration_install_url_response_installation_url(&self) -> bool {
        matches!(self, Self::CreateIntegrationInstallUrlResponseInstallationUrl(_))
    }

    pub fn is_create_integration_install_url_response_register_url(&self) -> bool {
        matches!(self, Self::CreateIntegrationInstallUrlResponseRegisterUrl(_))
    }


    pub fn as_create_integration_install_url_response_installation_url(&self) -> Option<&CreateIntegrationInstallUrlResponseInstallationUrl> {
        match self {
                    Self::CreateIntegrationInstallUrlResponseInstallationUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_integration_install_url_response_installation_url(self) -> Option<CreateIntegrationInstallUrlResponseInstallationUrl> {
        match self {
                    Self::CreateIntegrationInstallUrlResponseInstallationUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_create_integration_install_url_response_register_url(&self) -> Option<&CreateIntegrationInstallUrlResponseRegisterUrl> {
        match self {
                    Self::CreateIntegrationInstallUrlResponseRegisterUrl(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_integration_install_url_response_register_url(self) -> Option<CreateIntegrationInstallUrlResponseRegisterUrl> {
        match self {
                    Self::CreateIntegrationInstallUrlResponseRegisterUrl(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for CreateIntegrationInstallUrlResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateIntegrationInstallUrlResponseInstallationUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CreateIntegrationInstallUrlResponseRegisterUrl(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
