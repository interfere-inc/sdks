pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReleaseDestination {
        VercelReleaseDestination(VercelReleaseDestination),

        CloudflareReleaseDestination(CloudflareReleaseDestination),

        CliReleaseDestination(CliReleaseDestination),
}

impl ReleaseDestination {
    pub fn is_vercel_release_destination(&self) -> bool {
        matches!(self, Self::VercelReleaseDestination(_))
    }

    pub fn is_cloudflare_release_destination(&self) -> bool {
        matches!(self, Self::CloudflareReleaseDestination(_))
    }

    pub fn is_cli_release_destination(&self) -> bool {
        matches!(self, Self::CliReleaseDestination(_))
    }


    pub fn as_vercel_release_destination(&self) -> Option<&VercelReleaseDestination> {
        match self {
                    Self::VercelReleaseDestination(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_vercel_release_destination(self) -> Option<VercelReleaseDestination> {
        match self {
                    Self::VercelReleaseDestination(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_cloudflare_release_destination(&self) -> Option<&CloudflareReleaseDestination> {
        match self {
                    Self::CloudflareReleaseDestination(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_cloudflare_release_destination(self) -> Option<CloudflareReleaseDestination> {
        match self {
                    Self::CloudflareReleaseDestination(value) => Some(value),
                    _ => None,
                }
    }

    pub fn as_cli_release_destination(&self) -> Option<&CliReleaseDestination> {
        match self {
                    Self::CliReleaseDestination(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_cli_release_destination(self) -> Option<CliReleaseDestination> {
        match self {
                    Self::CliReleaseDestination(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for ReleaseDestination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VercelReleaseDestination(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CloudflareReleaseDestination(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
            Self::CliReleaseDestination(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
