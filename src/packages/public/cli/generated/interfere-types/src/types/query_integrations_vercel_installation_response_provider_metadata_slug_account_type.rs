pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsVercelInstallationResponseProviderMetadataSlugAccountType {
    #[serde(rename = "team")]
    Team,
}
impl fmt::Display for QueryIntegrationsVercelInstallationResponseProviderMetadataSlugAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Team => "team",
        };
        write!(f, "{}", s)
    }
}
