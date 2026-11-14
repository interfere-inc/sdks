pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryWorkspacesCurrentResponseDemoResetStatus {
    Running,
    Succeeded,
    Failed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryWorkspacesCurrentResponseDemoResetStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Running => serializer.serialize_str("RUNNING"),
            Self::Succeeded => serializer.serialize_str("SUCCEEDED"),
            Self::Failed => serializer.serialize_str("FAILED"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryWorkspacesCurrentResponseDemoResetStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "RUNNING" => Ok(Self::Running),
            "SUCCEEDED" => Ok(Self::Succeeded),
            "FAILED" => Ok(Self::Failed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryWorkspacesCurrentResponseDemoResetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Running => write!(f, "RUNNING"),
            Self::Succeeded => write!(f, "SUCCEEDED"),
            Self::Failed => write!(f, "FAILED"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
