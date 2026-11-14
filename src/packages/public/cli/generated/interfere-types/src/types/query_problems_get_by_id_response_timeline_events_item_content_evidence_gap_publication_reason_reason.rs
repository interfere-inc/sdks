pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason {
    EvidenceUnverified,
    IncompleteProposal,
    CheckUnavailable,
    ConflictsWithProblem,
    IncidentUnconfirmed,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EvidenceUnverified => serializer.serialize_str("evidence-unverified"),
            Self::IncompleteProposal => serializer.serialize_str("incomplete-proposal"),
            Self::CheckUnavailable => serializer.serialize_str("check-unavailable"),
            Self::ConflictsWithProblem => serializer.serialize_str("conflicts-with-problem"),
            Self::IncidentUnconfirmed => serializer.serialize_str("incident-unconfirmed"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "evidence-unverified" => Ok(Self::EvidenceUnverified),
            "incomplete-proposal" => Ok(Self::IncompleteProposal),
            "check-unavailable" => Ok(Self::CheckUnavailable),
            "conflicts-with-problem" => Ok(Self::ConflictsWithProblem),
            "incident-unconfirmed" => Ok(Self::IncidentUnconfirmed),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemContentEvidenceGapPublicationReasonReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EvidenceUnverified => write!(f, "evidence-unverified"),
            Self::IncompleteProposal => write!(f, "incomplete-proposal"),
            Self::CheckUnavailable => write!(f, "check-unavailable"),
            Self::ConflictsWithProblem => write!(f, "conflicts-with-problem"),
            Self::IncidentUnconfirmed => write!(f, "incident-unconfirmed"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
