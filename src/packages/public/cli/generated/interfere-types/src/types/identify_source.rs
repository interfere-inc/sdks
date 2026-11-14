pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum IdentifySource {
        #[serde(rename = "clerk")]
        #[non_exhaustive]
        Clerk {
            name: ClerkIdentifySourceName,
        },

        #[serde(rename = "auth0")]
        #[non_exhaustive]
        Auth0 {
            name: Auth0IdentifySourceName,
        },

        #[serde(rename = "workos")]
        #[non_exhaustive]
        Workos {
            name: WorkOsIdentifySourceName,
        },

        #[serde(rename = "custom")]
        #[non_exhaustive]
        Custom {
            #[serde(default)]
            name: String,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl IdentifySource {
    pub fn clerk(name: ClerkIdentifySourceName) -> Self {
        Self::Clerk { name }
    }

    pub fn auth0(name: Auth0IdentifySourceName) -> Self {
        Self::Auth0 { name }
    }

    pub fn workos(name: WorkOsIdentifySourceName) -> Self {
        Self::Workos { name }
    }

    pub fn custom(name: String) -> Self {
        Self::Custom { name }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
