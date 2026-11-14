pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DetectIntegrationsResponseIntegrationsItemInstallOperationId {
    #[serde(rename = "createIntegrationInstallUrl")]
    CreateIntegrationInstallUrl,
}
impl fmt::Display for DetectIntegrationsResponseIntegrationsItemInstallOperationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CreateIntegrationInstallUrl => "createIntegrationInstallUrl",
        };
        write!(f, "{}", s)
    }
}
