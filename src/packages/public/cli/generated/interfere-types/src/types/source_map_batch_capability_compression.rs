pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SourceMapBatchCapabilityCompression {
    #[serde(rename = "gzip")]
    Gzip,
}
impl fmt::Display for SourceMapBatchCapabilityCompression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Gzip => "gzip",
        };
        write!(f, "{}", s)
    }
}
