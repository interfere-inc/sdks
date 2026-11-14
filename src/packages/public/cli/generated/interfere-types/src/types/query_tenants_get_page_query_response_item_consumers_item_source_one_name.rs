pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneName {
    Auth0,
}
impl fmt::Display for QueryTenantsGetPageQueryResponseItemConsumersItemSourceOneName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auth0 => "Auth0",
        };
        write!(f, "{}", s)
    }
}
