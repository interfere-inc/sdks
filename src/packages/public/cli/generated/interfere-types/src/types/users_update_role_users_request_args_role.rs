pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateRoleUsersRequestArgsRole {
    OrgOwner,
    OrgAdmin,
    OrgMember,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateRoleUsersRequestArgsRole {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OrgOwner => serializer.serialize_str("org:owner"),
            Self::OrgAdmin => serializer.serialize_str("org:admin"),
            Self::OrgMember => serializer.serialize_str("org:member"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateRoleUsersRequestArgsRole {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "org:owner" => Ok(Self::OrgOwner),
            "org:admin" => Ok(Self::OrgAdmin),
            "org:member" => Ok(Self::OrgMember),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateRoleUsersRequestArgsRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OrgOwner => write!(f, "org:owner"),
            Self::OrgAdmin => write!(f, "org:admin"),
            Self::OrgMember => write!(f, "org:member"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
