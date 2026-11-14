pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Why the SDK started recording an anonymous session: error marks the buffer flushed when the visitor hit an error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateSessionRequestReplayTrigger {
    #[serde(rename = "error")]
    Error,
}
impl fmt::Display for CreateSessionRequestReplayTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Error => "error",
        };
        write!(f, "{}", s)
    }
}
