pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum ConsumerDetailsResponseSource {
        ConsumerDetailsResponseSourceZero(ConsumerDetailsResponseSourceZero),

        ConsumerDetailsResponseSourceOne(ConsumerDetailsResponseSourceOne),

        ConsumerDetailsResponseSourceTwo(ConsumerDetailsResponseSourceTwo),

        ConsumerDetailsResponseSourceThree(ConsumerDetailsResponseSourceThree),
}

impl ConsumerDetailsResponseSource {
    pub fn is_consumer_details_response_source_zero(&self) -> bool {
        matches!(self, Self::ConsumerDetailsResponseSourceZero(_))
    }

    pub fn is_consumer_details_response_source_one(&self) -> bool {
        matches!(self, Self::ConsumerDetailsResponseSourceOne(_))
    }

    pub fn is_consumer_details_response_source_two(&self) -> bool {
        matches!(self, Self::ConsumerDetailsResponseSourceTwo(_))
    }

    pub fn is_consumer_details_response_source_three(&self) -> bool {
        matches!(self, Self::ConsumerDetailsResponseSourceThree(_))
    }


    pub fn as_consumer_details_response_source_zero(&self) -> Option<&ConsumerDetailsResponseSourceZero> {
        match self {
                    Self::ConsumerDetailsResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_details_response_source_zero(self) -> Option<ConsumerDetailsResponseSourceZero> {
        match self {
                    Self::ConsumerDetailsResponseSourceZero(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_consumer_details_response_source_one(&self) -> Option<&ConsumerDetailsResponseSourceOne> {
        match self {
                    Self::ConsumerDetailsResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_details_response_source_one(self) -> Option<ConsumerDetailsResponseSourceOne> {
        match self {
                    Self::ConsumerDetailsResponseSourceOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_consumer_details_response_source_two(&self) -> Option<&ConsumerDetailsResponseSourceTwo> {
        match self {
                    Self::ConsumerDetailsResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_details_response_source_two(self) -> Option<ConsumerDetailsResponseSourceTwo> {
        match self {
                    Self::ConsumerDetailsResponseSourceTwo(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_consumer_details_response_source_three(&self) -> Option<&ConsumerDetailsResponseSourceThree> {
        match self {
                    Self::ConsumerDetailsResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_consumer_details_response_source_three(self) -> Option<ConsumerDetailsResponseSourceThree> {
        match self {
                    Self::ConsumerDetailsResponseSourceThree(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ConsumerDetailsResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConsumerDetailsResponseSourceZero(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConsumerDetailsResponseSourceOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConsumerDetailsResponseSourceTwo(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::ConsumerDetailsResponseSourceThree(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
