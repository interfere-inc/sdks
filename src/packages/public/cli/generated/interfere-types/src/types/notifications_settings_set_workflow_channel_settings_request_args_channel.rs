pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetWorkflowChannelSettingsRequestArgsChannel {
    Email,
    InAppFeed,
    Chat,
    Sms,
    Push,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SetWorkflowChannelSettingsRequestArgsChannel {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::InAppFeed => serializer.serialize_str("in_app_feed"),
            Self::Chat => serializer.serialize_str("chat"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::Push => serializer.serialize_str("push"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SetWorkflowChannelSettingsRequestArgsChannel {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "in_app_feed" => Ok(Self::InAppFeed),
            "chat" => Ok(Self::Chat),
            "sms" => Ok(Self::Sms),
            "push" => Ok(Self::Push),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SetWorkflowChannelSettingsRequestArgsChannel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::InAppFeed => write!(f, "in_app_feed"),
            Self::Chat => write!(f, "chat"),
            Self::Sms => write!(f, "sms"),
            Self::Push => write!(f, "push"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
