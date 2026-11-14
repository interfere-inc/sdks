pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus {
    Pending,
    Scanning,
    Ready,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Scanning => serializer.serialize_str("scanning"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Failed => serializer.serialize_str("failed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "scanning" => Ok(Self::Scanning),
            "ready" => Ok(Self::Ready),
            "failed" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryIntegrationsCloudflareInstallationResponseProviderMetadataThreeRepositoryScansValueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Scanning => write!(f, "scanning"),
            Self::Ready => write!(f, "ready"),
            Self::Failed => write!(f, "failed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
