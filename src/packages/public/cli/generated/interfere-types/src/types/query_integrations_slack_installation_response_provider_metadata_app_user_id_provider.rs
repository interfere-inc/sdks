pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdProvider {
    #[serde(rename = "linear")]
    Linear,
}
impl fmt::Display for QueryIntegrationsSlackInstallationResponseProviderMetadataAppUserIdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Linear => "linear",
        };
        write!(f, "{}", s)
    }
}
