pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConnectionProviderMetadataThreeProvider {
    #[serde(rename = "ghe_cloud")]
    GheCloud,
}
impl fmt::Display for ConnectionProviderMetadataThreeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::GheCloud => "ghe_cloud",
        };
        write!(f, "{}", s)
    }
}
