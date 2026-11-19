pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind {
    Noise,
    NotAMalfunction,
    WontFix,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Noise => serializer.serialize_str("noise"),
            Self::NotAMalfunction => serializer.serialize_str("notAMalfunction"),
            Self::WontFix => serializer.serialize_str("wontFix"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "noise" => Ok(Self::Noise),
            "notAMalfunction" => Ok(Self::NotAMalfunction),
            "wontFix" => Ok(Self::WontFix),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentDismissalDismissalZeroKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Noise => write!(f, "noise"),
            Self::NotAMalfunction => write!(f, "notAMalfunction"),
            Self::WontFix => write!(f, "wontFix"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
