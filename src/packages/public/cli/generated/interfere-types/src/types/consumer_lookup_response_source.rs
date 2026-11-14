pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ConsumerLookupResponseSource {
        ConsumerLookupResponseSourceZero(ConsumerLookupResponseSourceZero),

        ConsumerLookupResponseSourceOne(ConsumerLookupResponseSourceOne),

        ConsumerLookupResponseSourceTwo(ConsumerLookupResponseSourceTwo),

        ConsumerLookupResponseSourceThree(ConsumerLookupResponseSourceThree),
}

impl ConsumerLookupResponseSource {
    pub fn is_consumer_lookup_response_source_zero(&self) -> bool {
        matches!(self, Self::ConsumerLookupResponseSourceZero(_))
    }

    pub fn is_consumer_lookup_response_source_one(&self) -> bool {
        matches!(self, Self::ConsumerLookupResponseSourceOne(_))
    }

    pub fn is_consumer_lookup_response_source_two(&self) -> bool {
        matches!(self, Self::ConsumerLookupResponseSourceTwo(_))
    }

    pub fn is_consumer_lookup_response_source_three(&self) -> bool {
        matches!(self, Self::ConsumerLookupResponseSourceThree(_))
    }


    pub fn as_consumer_lookup_response_source_zero(&self) -> Option<&ConsumerLookupResponseSourceZero> {
        match self {
                    Self::ConsumerLookupResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_lookup_response_source_zero(self) -> Option<ConsumerLookupResponseSourceZero> {
        match self {
                    Self::ConsumerLookupResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_consumer_lookup_response_source_one(&self) -> Option<&ConsumerLookupResponseSourceOne> {
        match self {
                    Self::ConsumerLookupResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_lookup_response_source_one(self) -> Option<ConsumerLookupResponseSourceOne> {
        match self {
                    Self::ConsumerLookupResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_consumer_lookup_response_source_two(&self) -> Option<&ConsumerLookupResponseSourceTwo> {
        match self {
                    Self::ConsumerLookupResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_lookup_response_source_two(self) -> Option<ConsumerLookupResponseSourceTwo> {
        match self {
                    Self::ConsumerLookupResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_consumer_lookup_response_source_three(&self) -> Option<&ConsumerLookupResponseSourceThree> {
        match self {
                    Self::ConsumerLookupResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_lookup_response_source_three(self) -> Option<ConsumerLookupResponseSourceThree> {
        match self {
                    Self::ConsumerLookupResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConsumerLookupResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConsumerLookupResponseSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConsumerLookupResponseSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConsumerLookupResponseSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConsumerLookupResponseSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
