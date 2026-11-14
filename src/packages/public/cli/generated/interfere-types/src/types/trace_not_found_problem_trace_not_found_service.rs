pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TraceNotFoundProblemTraceNotFoundService {
    Collector,
    Gateway,
    Operation,
    Intelligence,
    FrontierAuth,
    FrontierDrain,
    FrontierRuntime,
    FrontierBuildtime,
    ApiFindings,
    ApiUpdates,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for TraceNotFoundProblemTraceNotFoundService {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Collector => serializer.serialize_str("collector"),
            Self::Gateway => serializer.serialize_str("gateway"),
            Self::Operation => serializer.serialize_str("operation"),
            Self::Intelligence => serializer.serialize_str("intelligence"),
            Self::FrontierAuth => serializer.serialize_str("frontier-auth"),
            Self::FrontierDrain => serializer.serialize_str("frontier-drain"),
            Self::FrontierRuntime => serializer.serialize_str("frontier-runtime"),
            Self::FrontierBuildtime => serializer.serialize_str("frontier-buildtime"),
            Self::ApiFindings => serializer.serialize_str("api-findings"),
            Self::ApiUpdates => serializer.serialize_str("api-updates"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for TraceNotFoundProblemTraceNotFoundService {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "collector" => Ok(Self::Collector),
            "gateway" => Ok(Self::Gateway),
            "operation" => Ok(Self::Operation),
            "intelligence" => Ok(Self::Intelligence),
            "frontier-auth" => Ok(Self::FrontierAuth),
            "frontier-drain" => Ok(Self::FrontierDrain),
            "frontier-runtime" => Ok(Self::FrontierRuntime),
            "frontier-buildtime" => Ok(Self::FrontierBuildtime),
            "api-findings" => Ok(Self::ApiFindings),
            "api-updates" => Ok(Self::ApiUpdates),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for TraceNotFoundProblemTraceNotFoundService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collector => write!(f, "collector"),
            Self::Gateway => write!(f, "gateway"),
            Self::Operation => write!(f, "operation"),
            Self::Intelligence => write!(f, "intelligence"),
            Self::FrontierAuth => write!(f, "frontier-auth"),
            Self::FrontierDrain => write!(f, "frontier-drain"),
            Self::FrontierRuntime => write!(f, "frontier-runtime"),
            Self::FrontierBuildtime => write!(f, "frontier-buildtime"),
            Self::ApiFindings => write!(f, "api-findings"),
            Self::ApiUpdates => write!(f, "api-updates"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
