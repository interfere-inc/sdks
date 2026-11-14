pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem {
    CollabProblemAssigned,
    CollabMention,
    CollabReaction,
    ProblemNeedsAttention,
    ProblemRouting,
    ProblemsDigest,
    ProblemsOpened,
    SubscriptionsComment,
    TeamMembershipRequest,
    WorkspaceBilling,
    WorkspaceIntegrationStatus,
    WorkspaceSurfaceBindingMissing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CollabProblemAssigned => serializer.serialize_str("collab-problem-assigned"),
            Self::CollabMention => serializer.serialize_str("collab-mention"),
            Self::CollabReaction => serializer.serialize_str("collab-reaction"),
            Self::ProblemNeedsAttention => serializer.serialize_str("problem-needs-attention"),
            Self::ProblemRouting => serializer.serialize_str("problem-routing"),
            Self::ProblemsDigest => serializer.serialize_str("problems-digest"),
            Self::ProblemsOpened => serializer.serialize_str("problems-opened"),
            Self::SubscriptionsComment => serializer.serialize_str("subscriptions-comment"),
            Self::TeamMembershipRequest => serializer.serialize_str("team-membership-request"),
            Self::WorkspaceBilling => serializer.serialize_str("workspace-billing"),
            Self::WorkspaceIntegrationStatus => serializer.serialize_str("workspace-integration-status"),
            Self::WorkspaceSurfaceBindingMissing => serializer.serialize_str("workspace-surface-binding-missing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "collab-problem-assigned" => Ok(Self::CollabProblemAssigned),
            "collab-mention" => Ok(Self::CollabMention),
            "collab-reaction" => Ok(Self::CollabReaction),
            "problem-needs-attention" => Ok(Self::ProblemNeedsAttention),
            "problem-routing" => Ok(Self::ProblemRouting),
            "problems-digest" => Ok(Self::ProblemsDigest),
            "problems-opened" => Ok(Self::ProblemsOpened),
            "subscriptions-comment" => Ok(Self::SubscriptionsComment),
            "team-membership-request" => Ok(Self::TeamMembershipRequest),
            "workspace-billing" => Ok(Self::WorkspaceBilling),
            "workspace-integration-status" => Ok(Self::WorkspaceIntegrationStatus),
            "workspace-surface-binding-missing" => Ok(Self::WorkspaceSurfaceBindingMissing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SetWorkflowChannelSettingsRequestArgsWorkflowKeysItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CollabProblemAssigned => write!(f, "collab-problem-assigned"),
            Self::CollabMention => write!(f, "collab-mention"),
            Self::CollabReaction => write!(f, "collab-reaction"),
            Self::ProblemNeedsAttention => write!(f, "problem-needs-attention"),
            Self::ProblemRouting => write!(f, "problem-routing"),
            Self::ProblemsDigest => write!(f, "problems-digest"),
            Self::ProblemsOpened => write!(f, "problems-opened"),
            Self::SubscriptionsComment => write!(f, "subscriptions-comment"),
            Self::TeamMembershipRequest => write!(f, "team-membership-request"),
            Self::WorkspaceBilling => write!(f, "workspace-billing"),
            Self::WorkspaceIntegrationStatus => write!(f, "workspace-integration-status"),
            Self::WorkspaceSurfaceBindingMissing => write!(f, "workspace-surface-binding-missing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
