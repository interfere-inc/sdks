pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnauthorizedErrorBodyOneCode {
    #[serde(rename = "AUTH_SESSION_INVALID")]
    AuthSessionInvalid,
}
impl fmt::Display for UnauthorizedErrorBodyOneCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AuthSessionInvalid => "AUTH_SESSION_INVALID",
        };
        write!(f, "{}", s)
    }
}
