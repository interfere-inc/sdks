pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConnectionProviderMetadataAssetsOnlyWorkersProvider {
    #[serde(rename = "cloudflare")]
    Cloudflare,
}
impl fmt::Display for ConnectionProviderMetadataAssetsOnlyWorkersProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cloudflare => "cloudflare",
        };
        write!(f, "{}", s)
    }
}
