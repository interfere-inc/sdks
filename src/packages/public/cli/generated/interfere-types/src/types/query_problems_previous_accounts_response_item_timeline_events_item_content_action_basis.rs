pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis {
    Account,
    Fingerprint,
    HttpRoute,
    SameRequest,
    SessionRoute,
    PageLoadRefusal,
    Manual,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Account => serializer.serialize_str("account"),
            Self::Fingerprint => serializer.serialize_str("fingerprint"),
            Self::HttpRoute => serializer.serialize_str("http-route"),
            Self::SameRequest => serializer.serialize_str("same-request"),
            Self::SessionRoute => serializer.serialize_str("session-route"),
            Self::PageLoadRefusal => serializer.serialize_str("page-load-refusal"),
            Self::Manual => serializer.serialize_str("manual"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account" => Ok(Self::Account),
            "fingerprint" => Ok(Self::Fingerprint),
            "http-route" => Ok(Self::HttpRoute),
            "same-request" => Ok(Self::SameRequest),
            "session-route" => Ok(Self::SessionRoute),
            "page-load-refusal" => Ok(Self::PageLoadRefusal),
            "manual" => Ok(Self::Manual),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentActionBasis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Account => write!(f, "account"),
            Self::Fingerprint => write!(f, "fingerprint"),
            Self::HttpRoute => write!(f, "http-route"),
            Self::SameRequest => write!(f, "same-request"),
            Self::SessionRoute => write!(f, "session-route"),
            Self::PageLoadRefusal => write!(f, "page-load-refusal"),
            Self::Manual => write!(f, "manual"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
