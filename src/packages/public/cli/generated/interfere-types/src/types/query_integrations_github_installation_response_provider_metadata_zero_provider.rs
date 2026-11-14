pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGithubInstallationResponseProviderMetadataZeroProvider {
    #[serde(rename = "opentelemetry")]
    Opentelemetry,
}
impl fmt::Display for QueryIntegrationsGithubInstallationResponseProviderMetadataZeroProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Opentelemetry => "opentelemetry",
        };
        write!(f, "{}", s)
    }
}
