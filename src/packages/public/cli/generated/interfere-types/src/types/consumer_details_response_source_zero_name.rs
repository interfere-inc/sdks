pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConsumerDetailsResponseSourceZeroName {
    Clerk,
}
impl fmt::Display for ConsumerDetailsResponseSourceZeroName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Clerk => "Clerk",
        };
        write!(f, "{}", s)
    }
}
