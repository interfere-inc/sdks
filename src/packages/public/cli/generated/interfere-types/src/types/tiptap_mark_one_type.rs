pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapMarkOneType {
    #[serde(rename = "italic")]
    Italic,
}
impl fmt::Display for TiptapMarkOneType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Italic => "italic",
        };
        write!(f, "{}", s)
    }
}
