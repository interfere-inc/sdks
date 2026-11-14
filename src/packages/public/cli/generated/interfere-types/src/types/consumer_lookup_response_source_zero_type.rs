pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConsumerLookupResponseSourceZeroType {
    #[serde(rename = "clerk")]
    Clerk,
}
impl fmt::Display for ConsumerLookupResponseSourceZeroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Clerk => "clerk",
        };
        write!(f, "{}", s)
    }
}
