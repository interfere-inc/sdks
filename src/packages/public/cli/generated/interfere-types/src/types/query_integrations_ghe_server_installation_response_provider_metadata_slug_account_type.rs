pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugAccountType {
    #[serde(rename = "team")]
    Team,
}
impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Team => "team",
        };
        write!(f, "{}", s)
    }
}
