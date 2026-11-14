pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserIdProvider {
    #[serde(rename = "linear")]
    Linear,
}
impl fmt::Display for QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAppUserIdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Linear => "linear",
        };
        write!(f, "{}", s)
    }
}
