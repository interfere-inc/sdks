pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProblemSourceResponseFailingCommitUnavailable {
    RepositoryUnlinked,
    CommitUnresolved,
    NoRecentFact,
    NoRelease,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProblemSourceResponseFailingCommitUnavailable {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::RepositoryUnlinked => serializer.serialize_str("repository_unlinked"),
            Self::CommitUnresolved => serializer.serialize_str("commit_unresolved"),
            Self::NoRecentFact => serializer.serialize_str("no_recent_fact"),
            Self::NoRelease => serializer.serialize_str("no_release"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProblemSourceResponseFailingCommitUnavailable {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "repository_unlinked" => Ok(Self::RepositoryUnlinked),
            "commit_unresolved" => Ok(Self::CommitUnresolved),
            "no_recent_fact" => Ok(Self::NoRecentFact),
            "no_release" => Ok(Self::NoRelease),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProblemSourceResponseFailingCommitUnavailable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RepositoryUnlinked => write!(f, "repository_unlinked"),
            Self::CommitUnresolved => write!(f, "commit_unresolved"),
            Self::NoRecentFact => write!(f, "no_recent_fact"),
            Self::NoRelease => write!(f, "no_release"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
