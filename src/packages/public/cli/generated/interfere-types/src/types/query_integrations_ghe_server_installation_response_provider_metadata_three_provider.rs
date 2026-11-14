pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeProvider {
    #[serde(rename = "ghe_cloud")]
    GheCloud,
}
impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadataThreeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GheCloud => "ghe_cloud",
        };
        write!(f, "{}", s)
    }
}
