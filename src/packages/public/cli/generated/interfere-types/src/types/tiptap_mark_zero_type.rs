pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapMarkZeroType {
    #[serde(rename = "bold")]
    Bold,
}
impl fmt::Display for TiptapMarkZeroType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Bold => "bold",
        };
        write!(f, "{}", s)
    }
}
