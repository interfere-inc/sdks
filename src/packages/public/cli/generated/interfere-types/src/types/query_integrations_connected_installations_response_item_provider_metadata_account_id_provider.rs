pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdProvider {
    #[serde(rename = "ghe_server")]
    GheServer,
}
impl fmt::Display for QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAccountIdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GheServer => "ghe_server",
        };
        write!(f, "{}", s)
    }
}
