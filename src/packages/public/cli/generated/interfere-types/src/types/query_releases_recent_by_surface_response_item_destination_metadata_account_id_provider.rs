pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountIdProvider {
    #[serde(rename = "cloudflare")]
    Cloudflare,
}
impl fmt::Display for QueryReleasesRecentBySurfaceResponseItemDestinationMetadataAccountIdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cloudflare => "cloudflare",
        };
        write!(f, "{}", s)
    }
}
