pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapDocumentContentItemZeroContentItemTwoType {
    #[serde(rename = "emoji")]
    Emoji,
}
impl fmt::Display for TiptapDocumentContentItemZeroContentItemTwoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Emoji => "emoji",
        };
        write!(f, "{}", s)
    }
}
