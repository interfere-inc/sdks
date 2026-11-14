pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryIntegrationsByIdResponseProviderMetadataAppUserIdModule1Mode {
    #[serde(rename = "individual-approval")]
    IndividualApproval,
}
impl fmt::Display for QueryIntegrationsByIdResponseProviderMetadataAppUserIdModule1Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IndividualApproval => "individual-approval",
        };
        write!(f, "{}", s)
    }
}
