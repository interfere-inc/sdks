pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRoutingProvider {
    #[serde(rename = "slack")]
    Slack,
}
impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadataChatRoutingProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Slack => "slack",
        };
        write!(f, "{}", s)
    }
}
