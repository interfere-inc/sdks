pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapMarkThreeType {
    #[serde(rename = "strike")]
    Strike,
}
impl fmt::Display for TiptapMarkThreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Strike => "strike",
        };
        write!(f, "{}", s)
    }
}
