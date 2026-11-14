pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapMarkTwoType {
    #[serde(rename = "code")]
    Code,
}
impl fmt::Display for TiptapMarkTwoType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Code => "code",
        };
        write!(f, "{}", s)
    }
}
