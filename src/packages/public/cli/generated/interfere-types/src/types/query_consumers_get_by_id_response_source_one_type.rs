pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryConsumersGetByIdResponseSourceOneType {
    #[serde(rename = "auth0")]
    Auth0,
}
impl fmt::Display for QueryConsumersGetByIdResponseSourceOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Auth0 => "auth0",
        };
        write!(f, "{}", s)
    }
}
