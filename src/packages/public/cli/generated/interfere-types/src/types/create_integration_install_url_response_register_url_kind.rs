pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateIntegrationInstallUrlResponseRegisterUrlKind {
    #[serde(rename = "register")]
    Register,
}
impl fmt::Display for CreateIntegrationInstallUrlResponseRegisterUrlKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Register => "register",
        };
        write!(f, "{}", s)
    }
}
