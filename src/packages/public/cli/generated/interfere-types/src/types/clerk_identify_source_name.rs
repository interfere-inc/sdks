pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClerkIdentifySourceName {
    Clerk,
}
impl fmt::Display for ClerkIdentifySourceName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Clerk => "Clerk",
        };
        write!(f, "{}", s)
    }
}
