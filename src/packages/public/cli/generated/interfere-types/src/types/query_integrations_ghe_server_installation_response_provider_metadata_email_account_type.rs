pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsGheServerInstallationResponseProviderMetadataEmailAccountType {
    #[serde(rename = "user")]
    User,
}
impl fmt::Display for QueryIntegrationsGheServerInstallationResponseProviderMetadataEmailAccountType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::User => "user",
        };
        write!(f, "{}", s)
    }
}
