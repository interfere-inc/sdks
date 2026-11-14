pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum QueryMembershipRequestsForUserResponseItemOrganizationEnabledAuthMethodsItem {
    EnterpriseSso,
    OauthGoogle,
    OauthGithub,
    OauthVercel,
    Passkey,
    EmailCode,
    Password,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for QueryMembershipRequestsForUserResponseItemOrganizationEnabledAuthMethodsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::EnterpriseSso => serializer.serialize_str("enterprise_sso"),
            Self::OauthGoogle => serializer.serialize_str("oauth_google"),
            Self::OauthGithub => serializer.serialize_str("oauth_github"),
            Self::OauthVercel => serializer.serialize_str("oauth_vercel"),
            Self::Passkey => serializer.serialize_str("passkey"),
            Self::EmailCode => serializer.serialize_str("email_code"),
            Self::Password => serializer.serialize_str("password"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for QueryMembershipRequestsForUserResponseItemOrganizationEnabledAuthMethodsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "enterprise_sso" => Ok(Self::EnterpriseSso),
            "oauth_google" => Ok(Self::OauthGoogle),
            "oauth_github" => Ok(Self::OauthGithub),
            "oauth_vercel" => Ok(Self::OauthVercel),
            "passkey" => Ok(Self::Passkey),
            "email_code" => Ok(Self::EmailCode),
            "password" => Ok(Self::Password),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for QueryMembershipRequestsForUserResponseItemOrganizationEnabledAuthMethodsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnterpriseSso => write!(f, "enterprise_sso"),
            Self::OauthGoogle => write!(f, "oauth_google"),
            Self::OauthGithub => write!(f, "oauth_github"),
            Self::OauthVercel => write!(f, "oauth_vercel"),
            Self::Passkey => write!(f, "passkey"),
            Self::EmailCode => write!(f, "email_code"),
            Self::Password => write!(f, "password"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
