pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TiptapDocumentContentItemZeroContentItemSixType {
    #[serde(rename = "observationRef")]
    ObservationRef,
}
impl fmt::Display for TiptapDocumentContentItemZeroContentItemSixType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ObservationRef => "observationRef",
        };
        write!(f, "{}", s)
    }
}
