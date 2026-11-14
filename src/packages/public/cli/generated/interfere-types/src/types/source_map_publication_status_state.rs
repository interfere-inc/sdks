pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SourceMapPublicationStatusState {
    Absent,
    Uploading,
    Accepted,
    Ready,
    Expiring,
    Expired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SourceMapPublicationStatusState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Absent => serializer.serialize_str("absent"),
            Self::Uploading => serializer.serialize_str("uploading"),
            Self::Accepted => serializer.serialize_str("accepted"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::Expiring => serializer.serialize_str("expiring"),
            Self::Expired => serializer.serialize_str("expired"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SourceMapPublicationStatusState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "absent" => Ok(Self::Absent),
            "uploading" => Ok(Self::Uploading),
            "accepted" => Ok(Self::Accepted),
            "ready" => Ok(Self::Ready),
            "expiring" => Ok(Self::Expiring),
            "expired" => Ok(Self::Expired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SourceMapPublicationStatusState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Absent => write!(f, "absent"),
            Self::Uploading => write!(f, "uploading"),
            Self::Accepted => write!(f, "accepted"),
            Self::Ready => write!(f, "ready"),
            Self::Expiring => write!(f, "expiring"),
            Self::Expired => write!(f, "expired"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
