pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum QueryConsumersGetByIdResponseSourceThreeType {
    #[serde(rename = "custom")]
    Custom,
}
impl fmt::Display for QueryConsumersGetByIdResponseSourceThreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Custom => "custom",
        };
        write!(f, "{}", s)
    }
}
