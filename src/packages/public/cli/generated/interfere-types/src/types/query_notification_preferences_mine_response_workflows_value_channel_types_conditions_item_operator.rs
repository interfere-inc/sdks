pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator {
    EqualTo,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    Contains,
    NotContains,
    Empty,
    NotEmpty,
    Exists,
    NotExists,
    ContainsAll,
    NotContainsAll,
    IsTimestamp,
    IsNotTimestamp,
    IsTimestampOnOrAfter,
    IsTimestampBefore,
    IsTimestampOnOrAfterDate,
    IsTimestampBeforeDate,
    IsTimestampBetween,
    IsBetween,
    IsAudienceMember,
    IsNotAudienceMember,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EqualTo => serializer.serialize_str("equal_to"),
            Self::NotEqualTo => serializer.serialize_str("not_equal_to"),
            Self::GreaterThan => serializer.serialize_str("greater_than"),
            Self::LessThan => serializer.serialize_str("less_than"),
            Self::GreaterThanOrEqualTo => serializer.serialize_str("greater_than_or_equal_to"),
            Self::LessThanOrEqualTo => serializer.serialize_str("less_than_or_equal_to"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::NotContains => serializer.serialize_str("not_contains"),
            Self::Empty => serializer.serialize_str("empty"),
            Self::NotEmpty => serializer.serialize_str("not_empty"),
            Self::Exists => serializer.serialize_str("exists"),
            Self::NotExists => serializer.serialize_str("not_exists"),
            Self::ContainsAll => serializer.serialize_str("contains_all"),
            Self::NotContainsAll => serializer.serialize_str("not_contains_all"),
            Self::IsTimestamp => serializer.serialize_str("is_timestamp"),
            Self::IsNotTimestamp => serializer.serialize_str("is_not_timestamp"),
            Self::IsTimestampOnOrAfter => serializer.serialize_str("is_timestamp_on_or_after"),
            Self::IsTimestampBefore => serializer.serialize_str("is_timestamp_before"),
            Self::IsTimestampOnOrAfterDate => serializer.serialize_str("is_timestamp_on_or_after_date"),
            Self::IsTimestampBeforeDate => serializer.serialize_str("is_timestamp_before_date"),
            Self::IsTimestampBetween => serializer.serialize_str("is_timestamp_between"),
            Self::IsBetween => serializer.serialize_str("is_between"),
            Self::IsAudienceMember => serializer.serialize_str("is_audience_member"),
            Self::IsNotAudienceMember => serializer.serialize_str("is_not_audience_member"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "equal_to" => Ok(Self::EqualTo),
            "not_equal_to" => Ok(Self::NotEqualTo),
            "greater_than" => Ok(Self::GreaterThan),
            "less_than" => Ok(Self::LessThan),
            "greater_than_or_equal_to" => Ok(Self::GreaterThanOrEqualTo),
            "less_than_or_equal_to" => Ok(Self::LessThanOrEqualTo),
            "contains" => Ok(Self::Contains),
            "not_contains" => Ok(Self::NotContains),
            "empty" => Ok(Self::Empty),
            "not_empty" => Ok(Self::NotEmpty),
            "exists" => Ok(Self::Exists),
            "not_exists" => Ok(Self::NotExists),
            "contains_all" => Ok(Self::ContainsAll),
            "not_contains_all" => Ok(Self::NotContainsAll),
            "is_timestamp" => Ok(Self::IsTimestamp),
            "is_not_timestamp" => Ok(Self::IsNotTimestamp),
            "is_timestamp_on_or_after" => Ok(Self::IsTimestampOnOrAfter),
            "is_timestamp_before" => Ok(Self::IsTimestampBefore),
            "is_timestamp_on_or_after_date" => Ok(Self::IsTimestampOnOrAfterDate),
            "is_timestamp_before_date" => Ok(Self::IsTimestampBeforeDate),
            "is_timestamp_between" => Ok(Self::IsTimestampBetween),
            "is_between" => Ok(Self::IsBetween),
            "is_audience_member" => Ok(Self::IsAudienceMember),
            "is_not_audience_member" => Ok(Self::IsNotAudienceMember),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryNotificationPreferencesMineResponseWorkflowsValueChannelTypesConditionsItemOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EqualTo => write!(f, "equal_to"),
            Self::NotEqualTo => write!(f, "not_equal_to"),
            Self::GreaterThan => write!(f, "greater_than"),
            Self::LessThan => write!(f, "less_than"),
            Self::GreaterThanOrEqualTo => write!(f, "greater_than_or_equal_to"),
            Self::LessThanOrEqualTo => write!(f, "less_than_or_equal_to"),
            Self::Contains => write!(f, "contains"),
            Self::NotContains => write!(f, "not_contains"),
            Self::Empty => write!(f, "empty"),
            Self::NotEmpty => write!(f, "not_empty"),
            Self::Exists => write!(f, "exists"),
            Self::NotExists => write!(f, "not_exists"),
            Self::ContainsAll => write!(f, "contains_all"),
            Self::NotContainsAll => write!(f, "not_contains_all"),
            Self::IsTimestamp => write!(f, "is_timestamp"),
            Self::IsNotTimestamp => write!(f, "is_not_timestamp"),
            Self::IsTimestampOnOrAfter => write!(f, "is_timestamp_on_or_after"),
            Self::IsTimestampBefore => write!(f, "is_timestamp_before"),
            Self::IsTimestampOnOrAfterDate => write!(f, "is_timestamp_on_or_after_date"),
            Self::IsTimestampBeforeDate => write!(f, "is_timestamp_before_date"),
            Self::IsTimestampBetween => write!(f, "is_timestamp_between"),
            Self::IsBetween => write!(f, "is_between"),
            Self::IsAudienceMember => write!(f, "is_audience_member"),
            Self::IsNotAudienceMember => write!(f, "is_not_audience_member"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
