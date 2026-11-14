pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum WorkOsIdentifySourceName {
    #[serde(rename = "WorkOS")]
    WorkOs,
}
impl fmt::Display for WorkOsIdentifySourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkOs => "WorkOS",
        };
        write!(f, "{}", s)
    }
}
