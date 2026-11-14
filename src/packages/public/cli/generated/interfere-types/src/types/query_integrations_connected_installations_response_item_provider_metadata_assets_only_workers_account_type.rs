pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkersAccountType {
    #[serde(rename = "account")]
    Account,
}
impl fmt::Display for QueryIntegrationsConnectedInstallationsResponseItemProviderMetadataAssetsOnlyWorkersAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Account => "account",
        };
        write!(f, "{}", s)
    }
}
