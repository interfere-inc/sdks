pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsInstallationsResponseItemProviderMetadataSlugAccountType {
    #[serde(rename = "team")]
    Team,
}
impl fmt::Display for QueryIntegrationsInstallationsResponseItemProviderMetadataSlugAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Team => "team",
        };
        write!(f, "{}", s)
    }
}
