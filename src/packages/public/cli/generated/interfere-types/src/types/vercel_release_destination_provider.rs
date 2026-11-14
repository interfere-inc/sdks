pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VercelReleaseDestinationProvider {
    #[serde(rename = "vercel")]
    Vercel,
}
impl fmt::Display for VercelReleaseDestinationProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Vercel => "vercel",
        };
        write!(f, "{}", s)
    }
}
