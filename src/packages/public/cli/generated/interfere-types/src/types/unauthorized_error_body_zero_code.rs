pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnauthorizedErrorBodyZeroCode {
    #[serde(rename = "AUTH_MISSING")]
    AuthMissing,
}
impl fmt::Display for UnauthorizedErrorBodyZeroCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AuthMissing => "AUTH_MISSING",
        };
        write!(f, "{}", s)
    }
}
