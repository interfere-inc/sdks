pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConflictErrorBodyOneDataField {
    #[serde(rename = "name")]
    Name,
}
impl fmt::Display for ConflictErrorBodyOneDataField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Name => "name",
        };
        write!(f, "{}", s)
    }
}
