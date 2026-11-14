pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroProvider {
    #[serde(rename = "opentelemetry")]
    Opentelemetry,
}
impl fmt::Display for QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataZeroProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Opentelemetry => "opentelemetry",
        };
        write!(f, "{}", s)
    }
}
