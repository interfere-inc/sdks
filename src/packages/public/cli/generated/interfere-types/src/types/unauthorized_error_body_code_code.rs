pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnauthorizedErrorBodyCodeCode {
    #[serde(rename = "AUTH_ORG_REQUIRED")]
    AuthOrgRequired,
}
impl fmt::Display for UnauthorizedErrorBodyCodeCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AuthOrgRequired => "AUTH_ORG_REQUIRED",
        };
        write!(f, "{}", s)
    }
}
