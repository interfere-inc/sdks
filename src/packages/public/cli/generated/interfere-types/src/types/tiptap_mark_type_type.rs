pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapMarkTypeType {
    #[serde(rename = "highlight")]
    Highlight,
}
impl fmt::Display for TiptapMarkTypeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Highlight => "highlight",
        };
        write!(f, "{}", s)
    }
}
