pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ConnectionProviderMetadata {
        ConnectionProviderMetadataZero(ConnectionProviderMetadataZero),

        ConnectionProviderMetadataAuthentication(ConnectionProviderMetadataAuthentication),

        ConnectionProviderMetadataTwo(ConnectionProviderMetadataTwo),

        ConnectionProviderMetadataThree(ConnectionProviderMetadataThree),

        ConnectionProviderMetadataAccountId(ConnectionProviderMetadataAccountId),

        ConnectionProviderMetadataSlug(ConnectionProviderMetadataSlug),

        ConnectionProviderMetadataEmail(ConnectionProviderMetadataEmail),

        ConnectionProviderMetadataAssetsOnlyWorkers(ConnectionProviderMetadataAssetsOnlyWorkers),

        ConnectionProviderMetadataChatRouting(ConnectionProviderMetadataChatRouting),

        ConnectionProviderMetadataNine(ConnectionProviderMetadataNine),

        ConnectionProviderMetadataAppUserId(ConnectionProviderMetadataAppUserId),
}

impl ConnectionProviderMetadata {
    pub fn is_connection_provider_metadata_zero(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataZero(_))
    }

    pub fn is_connection_provider_metadata_authentication(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataAuthentication(_))
    }

    pub fn is_connection_provider_metadata_two(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataTwo(_))
    }

    pub fn is_connection_provider_metadata_three(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataThree(_))
    }

    pub fn is_connection_provider_metadata_account_id(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataAccountId(_))
    }

    pub fn is_connection_provider_metadata_slug(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataSlug(_))
    }

    pub fn is_connection_provider_metadata_email(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataEmail(_))
    }

    pub fn is_connection_provider_metadata_assets_only_workers(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataAssetsOnlyWorkers(_))
    }

    pub fn is_connection_provider_metadata_chat_routing(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataChatRouting(_))
    }

    pub fn is_connection_provider_metadata_nine(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataNine(_))
    }

    pub fn is_connection_provider_metadata_app_user_id(&self) -> bool {
        matches!(self, Self::ConnectionProviderMetadataAppUserId(_))
    }


    pub fn as_connection_provider_metadata_zero(&self) -> Option<&ConnectionProviderMetadataZero> {
        match self {
                    Self::ConnectionProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_zero(self) -> Option<ConnectionProviderMetadataZero> {
        match self {
                    Self::ConnectionProviderMetadataZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_authentication(&self) -> Option<&ConnectionProviderMetadataAuthentication> {
        match self {
                    Self::ConnectionProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_authentication(self) -> Option<ConnectionProviderMetadataAuthentication> {
        match self {
                    Self::ConnectionProviderMetadataAuthentication(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_two(&self) -> Option<&ConnectionProviderMetadataTwo> {
        match self {
                    Self::ConnectionProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_two(self) -> Option<ConnectionProviderMetadataTwo> {
        match self {
                    Self::ConnectionProviderMetadataTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_three(&self) -> Option<&ConnectionProviderMetadataThree> {
        match self {
                    Self::ConnectionProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_three(self) -> Option<ConnectionProviderMetadataThree> {
        match self {
                    Self::ConnectionProviderMetadataThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_account_id(&self) -> Option<&ConnectionProviderMetadataAccountId> {
        match self {
                    Self::ConnectionProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_account_id(self) -> Option<ConnectionProviderMetadataAccountId> {
        match self {
                    Self::ConnectionProviderMetadataAccountId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_slug(&self) -> Option<&ConnectionProviderMetadataSlug> {
        match self {
                    Self::ConnectionProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_slug(self) -> Option<ConnectionProviderMetadataSlug> {
        match self {
                    Self::ConnectionProviderMetadataSlug(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_email(&self) -> Option<&ConnectionProviderMetadataEmail> {
        match self {
                    Self::ConnectionProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_email(self) -> Option<ConnectionProviderMetadataEmail> {
        match self {
                    Self::ConnectionProviderMetadataEmail(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_assets_only_workers(&self) -> Option<&ConnectionProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::ConnectionProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_assets_only_workers(self) -> Option<ConnectionProviderMetadataAssetsOnlyWorkers> {
        match self {
                    Self::ConnectionProviderMetadataAssetsOnlyWorkers(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_chat_routing(&self) -> Option<&ConnectionProviderMetadataChatRouting> {
        match self {
                    Self::ConnectionProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_chat_routing(self) -> Option<ConnectionProviderMetadataChatRouting> {
        match self {
                    Self::ConnectionProviderMetadataChatRouting(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_nine(&self) -> Option<&ConnectionProviderMetadataNine> {
        match self {
                    Self::ConnectionProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_nine(self) -> Option<ConnectionProviderMetadataNine> {
        match self {
                    Self::ConnectionProviderMetadataNine(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_connection_provider_metadata_app_user_id(&self) -> Option<&ConnectionProviderMetadataAppUserId> {
        match self {
                    Self::ConnectionProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_connection_provider_metadata_app_user_id(self) -> Option<ConnectionProviderMetadataAppUserId> {
        match self {
                    Self::ConnectionProviderMetadataAppUserId(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConnectionProviderMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConnectionProviderMetadataZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataAuthentication(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataAccountId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataSlug(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataEmail(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataAssetsOnlyWorkers(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataChatRouting(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataNine(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConnectionProviderMetadataAppUserId(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
