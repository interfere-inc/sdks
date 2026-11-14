pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsSlackInstallationResponseProviderMetadataNineProvider {
    #[serde(rename = "cli")]
    Cli,
}
impl fmt::Display for QueryIntegrationsSlackInstallationResponseProviderMetadataNineProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cli => "cli",
        };
        write!(f, "{}", s)
    }
}
