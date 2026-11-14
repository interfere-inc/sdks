pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateIntegrationInstallUrlResponseInstallationUrlKind {
    #[serde(rename = "install")]
    Install,
}
impl fmt::Display for CreateIntegrationInstallUrlResponseInstallationUrlKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Install => "install",
        };
        write!(f, "{}", s)
    }
}
