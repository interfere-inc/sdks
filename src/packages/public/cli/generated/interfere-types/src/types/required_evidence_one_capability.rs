pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RequiredEvidenceOneCapability {
    #[serde(rename = "replay")]
    Replay,
}
impl fmt::Display for RequiredEvidenceOneCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Replay => "replay",
        };
        write!(f, "{}", s)
    }
}
