pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConnectionProviderMetadataTwoProvider {
    #[serde(rename = "github")]
    Github,
}
impl fmt::Display for ConnectionProviderMetadataTwoProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Github => "github",
        };
        write!(f, "{}", s)
    }
}
