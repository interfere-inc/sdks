pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapMarkFourType {
    #[serde(rename = "underline")]
    Underline,
}
impl fmt::Display for TiptapMarkFourType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Underline => "underline",
        };
        write!(f, "{}", s)
    }
}
