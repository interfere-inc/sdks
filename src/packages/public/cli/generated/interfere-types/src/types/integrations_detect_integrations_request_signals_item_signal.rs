pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A signal id from the catalogue this operation returns. Unknown ids are rejected.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DetectIntegrationsRequestSignalsItemSignal {
    GithubRemote,
    GithubWorkflows,
    GheCloudRemote,
    GheServerRemote,
    VercelConfig,
    VercelCiEnv,
    WranglerConfig,
    CloudflareCiEnv,
    SlackEnv,
    SlackUrl,
    SlackCiStep,
    LinearUrl,
    LinearEnv,
    LinearIssueIds,
    SentryConfig,
    PagerdutyConfig,
    JiraUrl,
    DiscordWebhook,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DetectIntegrationsRequestSignalsItemSignal {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GithubRemote => serializer.serialize_str("github_remote"),
            Self::GithubWorkflows => serializer.serialize_str("github_workflows"),
            Self::GheCloudRemote => serializer.serialize_str("ghe_cloud_remote"),
            Self::GheServerRemote => serializer.serialize_str("ghe_server_remote"),
            Self::VercelConfig => serializer.serialize_str("vercel_config"),
            Self::VercelCiEnv => serializer.serialize_str("vercel_ci_env"),
            Self::WranglerConfig => serializer.serialize_str("wrangler_config"),
            Self::CloudflareCiEnv => serializer.serialize_str("cloudflare_ci_env"),
            Self::SlackEnv => serializer.serialize_str("slack_env"),
            Self::SlackUrl => serializer.serialize_str("slack_url"),
            Self::SlackCiStep => serializer.serialize_str("slack_ci_step"),
            Self::LinearUrl => serializer.serialize_str("linear_url"),
            Self::LinearEnv => serializer.serialize_str("linear_env"),
            Self::LinearIssueIds => serializer.serialize_str("linear_issue_ids"),
            Self::SentryConfig => serializer.serialize_str("sentry_config"),
            Self::PagerdutyConfig => serializer.serialize_str("pagerduty_config"),
            Self::JiraUrl => serializer.serialize_str("jira_url"),
            Self::DiscordWebhook => serializer.serialize_str("discord_webhook"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DetectIntegrationsRequestSignalsItemSignal {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "github_remote" => Ok(Self::GithubRemote),
            "github_workflows" => Ok(Self::GithubWorkflows),
            "ghe_cloud_remote" => Ok(Self::GheCloudRemote),
            "ghe_server_remote" => Ok(Self::GheServerRemote),
            "vercel_config" => Ok(Self::VercelConfig),
            "vercel_ci_env" => Ok(Self::VercelCiEnv),
            "wrangler_config" => Ok(Self::WranglerConfig),
            "cloudflare_ci_env" => Ok(Self::CloudflareCiEnv),
            "slack_env" => Ok(Self::SlackEnv),
            "slack_url" => Ok(Self::SlackUrl),
            "slack_ci_step" => Ok(Self::SlackCiStep),
            "linear_url" => Ok(Self::LinearUrl),
            "linear_env" => Ok(Self::LinearEnv),
            "linear_issue_ids" => Ok(Self::LinearIssueIds),
            "sentry_config" => Ok(Self::SentryConfig),
            "pagerduty_config" => Ok(Self::PagerdutyConfig),
            "jira_url" => Ok(Self::JiraUrl),
            "discord_webhook" => Ok(Self::DiscordWebhook),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DetectIntegrationsRequestSignalsItemSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GithubRemote => write!(f, "github_remote"),
            Self::GithubWorkflows => write!(f, "github_workflows"),
            Self::GheCloudRemote => write!(f, "ghe_cloud_remote"),
            Self::GheServerRemote => write!(f, "ghe_server_remote"),
            Self::VercelConfig => write!(f, "vercel_config"),
            Self::VercelCiEnv => write!(f, "vercel_ci_env"),
            Self::WranglerConfig => write!(f, "wrangler_config"),
            Self::CloudflareCiEnv => write!(f, "cloudflare_ci_env"),
            Self::SlackEnv => write!(f, "slack_env"),
            Self::SlackUrl => write!(f, "slack_url"),
            Self::SlackCiStep => write!(f, "slack_ci_step"),
            Self::LinearUrl => write!(f, "linear_url"),
            Self::LinearEnv => write!(f, "linear_env"),
            Self::LinearIssueIds => write!(f, "linear_issue_ids"),
            Self::SentryConfig => write!(f, "sentry_config"),
            Self::PagerdutyConfig => write!(f, "pagerduty_config"),
            Self::JiraUrl => write!(f, "jira_url"),
            Self::DiscordWebhook => write!(f, "discord_webhook"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
