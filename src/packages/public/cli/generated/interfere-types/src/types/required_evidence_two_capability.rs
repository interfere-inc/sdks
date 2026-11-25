pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RequiredEvidenceTwoCapability {
    #[serde(rename = "user-attribution")]
    UserAttribution,
}
impl fmt::Display for RequiredEvidenceTwoCapability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::UserAttribution => "user-attribution",
        };
        write!(f, "{}", s)
    }
}
