pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsInstallationsResponseItemProviderMetadataChatRoutingProvider {
    #[serde(rename = "slack")]
    Slack,
}
impl fmt::Display for QueryIntegrationsInstallationsResponseItemProviderMetadataChatRoutingProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Slack => "slack",
        };
        write!(f, "{}", s)
    }
}
