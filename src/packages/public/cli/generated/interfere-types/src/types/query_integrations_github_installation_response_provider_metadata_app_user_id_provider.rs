pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdProvider {
    #[serde(rename = "linear")]
    Linear,
}
impl fmt::Display for QueryIntegrationsGithubInstallationResponseProviderMetadataAppUserIdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Linear => "linear",
        };
        write!(f, "{}", s)
    }
}
