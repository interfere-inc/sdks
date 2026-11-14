pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkersProvider {
    #[serde(rename = "cloudflare")]
    Cloudflare,
}
impl fmt::Display for QueryIntegrationsByIdResponseProviderMetadataAssetsOnlyWorkersProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cloudflare => "cloudflare",
        };
        write!(f, "{}", s)
    }
}
