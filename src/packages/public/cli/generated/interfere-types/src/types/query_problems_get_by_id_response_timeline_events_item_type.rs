pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryProblemsGetByIdResponseTimelineEventsItemType {
    ProblemDetected,
    DescriptionSet,
    Comment,
    ActionRequest,
    PriorityChanged,
    CategoryChanged,
    AssignmentChanged,
    StatusChanged,
    SourceLinked,
    RegressionDetected,
    AgentNarrative,
    Observation,
    RootCause,
    Fix,
    TitleSet,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryProblemsGetByIdResponseTimelineEventsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ProblemDetected => serializer.serialize_str("PROBLEM_DETECTED"),
            Self::DescriptionSet => serializer.serialize_str("DESCRIPTION_SET"),
            Self::Comment => serializer.serialize_str("COMMENT"),
            Self::ActionRequest => serializer.serialize_str("ACTION_REQUEST"),
            Self::PriorityChanged => serializer.serialize_str("PRIORITY_CHANGED"),
            Self::CategoryChanged => serializer.serialize_str("CATEGORY_CHANGED"),
            Self::AssignmentChanged => serializer.serialize_str("ASSIGNMENT_CHANGED"),
            Self::StatusChanged => serializer.serialize_str("STATUS_CHANGED"),
            Self::SourceLinked => serializer.serialize_str("SOURCE_LINKED"),
            Self::RegressionDetected => serializer.serialize_str("REGRESSION_DETECTED"),
            Self::AgentNarrative => serializer.serialize_str("AGENT_NARRATIVE"),
            Self::Observation => serializer.serialize_str("OBSERVATION"),
            Self::RootCause => serializer.serialize_str("ROOT_CAUSE"),
            Self::Fix => serializer.serialize_str("FIX"),
            Self::TitleSet => serializer.serialize_str("TITLE_SET"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryProblemsGetByIdResponseTimelineEventsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PROBLEM_DETECTED" => Ok(Self::ProblemDetected),
            "DESCRIPTION_SET" => Ok(Self::DescriptionSet),
            "COMMENT" => Ok(Self::Comment),
            "ACTION_REQUEST" => Ok(Self::ActionRequest),
            "PRIORITY_CHANGED" => Ok(Self::PriorityChanged),
            "CATEGORY_CHANGED" => Ok(Self::CategoryChanged),
            "ASSIGNMENT_CHANGED" => Ok(Self::AssignmentChanged),
            "STATUS_CHANGED" => Ok(Self::StatusChanged),
            "SOURCE_LINKED" => Ok(Self::SourceLinked),
            "REGRESSION_DETECTED" => Ok(Self::RegressionDetected),
            "AGENT_NARRATIVE" => Ok(Self::AgentNarrative),
            "OBSERVATION" => Ok(Self::Observation),
            "ROOT_CAUSE" => Ok(Self::RootCause),
            "FIX" => Ok(Self::Fix),
            "TITLE_SET" => Ok(Self::TitleSet),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryProblemsGetByIdResponseTimelineEventsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProblemDetected => write!(f, "PROBLEM_DETECTED"),
            Self::DescriptionSet => write!(f, "DESCRIPTION_SET"),
            Self::Comment => write!(f, "COMMENT"),
            Self::ActionRequest => write!(f, "ACTION_REQUEST"),
            Self::PriorityChanged => write!(f, "PRIORITY_CHANGED"),
            Self::CategoryChanged => write!(f, "CATEGORY_CHANGED"),
            Self::AssignmentChanged => write!(f, "ASSIGNMENT_CHANGED"),
            Self::StatusChanged => write!(f, "STATUS_CHANGED"),
            Self::SourceLinked => write!(f, "SOURCE_LINKED"),
            Self::RegressionDetected => write!(f, "REGRESSION_DETECTED"),
            Self::AgentNarrative => write!(f, "AGENT_NARRATIVE"),
            Self::Observation => write!(f, "OBSERVATION"),
            Self::RootCause => write!(f, "ROOT_CAUSE"),
            Self::Fix => write!(f, "FIX"),
            Self::TitleSet => write!(f, "TITLE_SET"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
