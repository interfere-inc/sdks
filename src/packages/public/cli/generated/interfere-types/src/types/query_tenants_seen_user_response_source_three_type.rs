pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryTenantsSeenUserResponseSourceThreeType {
    #[serde(rename = "custom")]
    Custom,
}
impl fmt::Display for QueryTenantsSeenUserResponseSourceThreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Custom => "custom",
        };
        write!(f, "{}", s)
    }
}
