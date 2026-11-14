pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineProvider {
    #[serde(rename = "cli")]
    Cli,
}
impl fmt::Display for QueryIntegrationsCloudflareInstallationResponseProviderMetadataNineProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cli => "cli",
        };
        write!(f, "{}", s)
    }
}
