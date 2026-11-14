pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsSlackInstallationResponseProviderMetadataSlugAccountType {
    #[serde(rename = "team")]
    Team,
}
impl fmt::Display for QueryIntegrationsSlackInstallationResponseProviderMetadataSlugAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Team => "team",
        };
        write!(f, "{}", s)
    }
}
