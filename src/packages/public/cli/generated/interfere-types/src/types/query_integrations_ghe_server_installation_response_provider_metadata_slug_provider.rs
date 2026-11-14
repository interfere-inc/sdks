pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugProvider {
    #[serde(rename = "vercel")]
    Vercel,
}
impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadataSlugProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vercel => "vercel",
        };
        write!(f, "{}", s)
    }
}
