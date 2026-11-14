pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConfigUnavailableProblemCode {
    #[serde(rename = "INGEST_INFRA_UNAVAILABLE")]
    IngestInfraUnavailable,
}
impl fmt::Display for ConfigUnavailableProblemCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::IngestInfraUnavailable => "INGEST_INFRA_UNAVAILABLE",
        };
        write!(f, "{}", s)
    }
}
