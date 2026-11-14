pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "provider")]
#[non_exhaustive]
pub enum ReleaseSource {
        #[serde(rename = "github")]
        #[non_exhaustive]
        Github {
            #[serde(rename = "authorLogin")]
            #[serde(skip_serializing_if = "Option::is_none")]
            author_login: Option<String>,
            #[serde(rename = "authorAvatarUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            author_avatar_url: Option<String>,
            #[serde(default)]
            branch: String,
            #[serde(rename = "commitMessage")]
            #[serde(default)]
            commit_message: String,
            #[serde(rename = "commitSha")]
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_sha: Option<String>,
            #[serde(rename = "runId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            run_id: Option<String>,
            #[serde(rename = "runUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            run_url: Option<String>,
            #[serde(rename = "runnerName")]
            #[serde(skip_serializing_if = "Option::is_none")]
            runner_name: Option<String>,
        },

        #[serde(rename = "cli")]
        #[non_exhaustive]
        Cli {
            #[serde(rename = "commitMessage")]
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_message: Option<String>,
            #[serde(rename = "commitRef")]
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_ref: Option<String>,
            #[serde(rename = "commitSha")]
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_sha: Option<String>,
            #[serde(rename = "commitUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            commit_url: Option<String>,
            #[serde(rename = "deploymentUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            deployment_url: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            environment: Option<String>,
            #[serde(rename = "runId")]
            #[serde(skip_serializing_if = "Option::is_none")]
            run_id: Option<String>,
            #[serde(rename = "runUrl")]
            #[serde(skip_serializing_if = "Option::is_none")]
            run_url: Option<String>,
            #[serde(rename = "runnerName")]
            #[serde(skip_serializing_if = "Option::is_none")]
            runner_name: Option<String>,
        },

        /// Catch-all variant for unrecognized discriminant values.
        /// If the server sends a discriminant not recognized by the current SDK
        /// version, the raw payload is captured here so callers can still inspect it.
        #[serde(untagged)]
        __Unknown(serde_json::Value),
}

impl ReleaseSource {
    pub fn github(branch: String, commit_message: String) -> Self {
        Self::Github { author_login: None, author_avatar_url: None, branch, commit_message, commit_sha: None, run_id: None, run_url: None, runner_name: None }
    }

    pub fn cli() -> Self {
        Self::Cli { commit_message: None, commit_ref: None, commit_sha: None, commit_url: None, deployment_url: None, environment: None, run_id: None, run_url: None, runner_name: None }
    }

    pub fn github_with_author_login(author_login: String, author_avatar_url: Option<String>, branch: String, commit_message: String, commit_sha: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Github { author_login: Some(author_login), author_avatar_url, branch, commit_message, commit_sha, run_id, run_url, runner_name }
    }

    pub fn github_with_author_avatar_url(author_login: Option<String>, author_avatar_url: String, branch: String, commit_message: String, commit_sha: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Github { author_login, author_avatar_url: Some(author_avatar_url), branch, commit_message, commit_sha, run_id, run_url, runner_name }
    }

    pub fn github_with_commit_sha(author_login: Option<String>, author_avatar_url: Option<String>, branch: String, commit_message: String, commit_sha: String, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Github { author_login, author_avatar_url, branch, commit_message, commit_sha: Some(commit_sha), run_id, run_url, runner_name }
    }

    pub fn github_with_run_id(author_login: Option<String>, author_avatar_url: Option<String>, branch: String, commit_message: String, commit_sha: Option<String>, run_id: String, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Github { author_login, author_avatar_url, branch, commit_message, commit_sha, run_id: Some(run_id), run_url, runner_name }
    }

    pub fn github_with_run_url(author_login: Option<String>, author_avatar_url: Option<String>, branch: String, commit_message: String, commit_sha: Option<String>, run_id: Option<String>, run_url: String, runner_name: Option<String>) -> Self {
        Self::Github { author_login, author_avatar_url, branch, commit_message, commit_sha, run_id, run_url: Some(run_url), runner_name }
    }

    pub fn github_with_runner_name(author_login: Option<String>, author_avatar_url: Option<String>, branch: String, commit_message: String, commit_sha: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: String) -> Self {
        Self::Github { author_login, author_avatar_url, branch, commit_message, commit_sha, run_id, run_url, runner_name: Some(runner_name) }
    }

    pub fn cli_with_commit_message(commit_message: String, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: Option<String>, environment: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message: Some(commit_message), commit_ref, commit_sha, commit_url, deployment_url, environment, run_id, run_url, runner_name }
    }

    pub fn cli_with_commit_ref(commit_message: Option<String>, commit_ref: String, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: Option<String>, environment: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref: Some(commit_ref), commit_sha, commit_url, deployment_url, environment, run_id, run_url, runner_name }
    }

    pub fn cli_with_commit_sha(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: String, commit_url: Option<String>, deployment_url: Option<String>, environment: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha: Some(commit_sha), commit_url, deployment_url, environment, run_id, run_url, runner_name }
    }

    pub fn cli_with_commit_url(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: String, deployment_url: Option<String>, environment: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha, commit_url: Some(commit_url), deployment_url, environment, run_id, run_url, runner_name }
    }

    pub fn cli_with_deployment_url(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: String, environment: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha, commit_url, deployment_url: Some(deployment_url), environment, run_id, run_url, runner_name }
    }

    pub fn cli_with_environment(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: Option<String>, environment: String, run_id: Option<String>, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha, commit_url, deployment_url, environment: Some(environment), run_id, run_url, runner_name }
    }

    pub fn cli_with_run_id(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: Option<String>, environment: Option<String>, run_id: String, run_url: Option<String>, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha, commit_url, deployment_url, environment, run_id: Some(run_id), run_url, runner_name }
    }

    pub fn cli_with_run_url(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: Option<String>, environment: Option<String>, run_id: Option<String>, run_url: String, runner_name: Option<String>) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha, commit_url, deployment_url, environment, run_id, run_url: Some(run_url), runner_name }
    }

    pub fn cli_with_runner_name(commit_message: Option<String>, commit_ref: Option<String>, commit_sha: Option<String>, commit_url: Option<String>, deployment_url: Option<String>, environment: Option<String>, run_id: Option<String>, run_url: Option<String>, runner_name: String) -> Self {
        Self::Cli { commit_message, commit_ref, commit_sha, commit_url, deployment_url, environment, run_id, run_url, runner_name: Some(runner_name) }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
