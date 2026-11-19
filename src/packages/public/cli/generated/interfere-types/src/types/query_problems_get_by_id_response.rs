pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryProblemsGetByIdResponse {
    #[serde(rename = "affectedUsers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affected_users: Option<i64>,
    #[serde(rename = "assigneeUserId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_user_id: Option<String>,
    #[serde(rename = "awaitingInputSince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub awaiting_input_since: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<QueryProblemsGetByIdResponseCategory>,
    #[serde(rename = "categoryTaintedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub category_tainted_at: Option<f64>,
    #[serde(rename = "compactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "descriptionTaintedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub description_tainted_at: Option<f64>,
    #[serde(rename = "dismissReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismiss_reason: Option<String>,
    #[serde(rename = "dismissedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub dismissed_at: Option<f64>,
    #[serde(rename = "dismissedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_by: Option<String>,
    #[serde(rename = "dismissedDuplicateOfId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dismissed_duplicate_of_id: Option<String>,
    #[serde(rename = "duplicateOfId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_of_id: Option<String>,
    #[serde(default)]
    pub fingerprint: String,
    #[serde(rename = "firstSeenAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub first_seen_at: f64,
    #[serde(rename = "hasLiveFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_live_findings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "investigationOutcome")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub investigation_outcome: Option<String>,
    #[serde(rename = "lastError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    #[serde(rename = "lastSeenAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub last_seen_at: f64,
    #[serde(rename = "nextRecoveryCheckAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub next_recovery_check_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurrences: Option<i64>,
    #[serde(rename = "organizationId")]
    #[serde(default)]
    pub organization_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<QueryProblemsGetByIdResponsePriority>,
    #[serde(rename = "priorityFloor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_floor: Option<QueryProblemsGetByIdResponsePriorityFloor>,
    #[serde(rename = "priorityFloorBasis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority_floor_basis: Option<QueryProblemsGetByIdResponsePriorityFloorBasis>,
    #[serde(rename = "priorityTaintedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub priority_tainted_at: Option<f64>,
    #[serde(rename = "regressionProbability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub regression_probability: Option<f64>,
    #[serde(rename = "resolvedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub resolved_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<i64>,
    #[serde(rename = "serviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    pub status: QueryProblemsGetByIdResponseStatus,
    #[serde(rename = "surfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "titleTaintedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub title_tainted_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub created_at: f64,
    #[serde(rename = "deletedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub deleted_at: Option<f64>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub updated_at: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<QueryProblemsGetByIdResponseSurface>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<QueryProblemsGetByIdResponseAssignee>,
    #[serde(rename = "timelineEvents")]
    #[serde(default)]
    pub timeline_events: Vec<QueryProblemsGetByIdResponseTimelineEventsItem>,
}

impl QueryProblemsGetByIdResponse {
    pub fn builder() -> QueryProblemsGetByIdResponseBuilder {
        <QueryProblemsGetByIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsGetByIdResponseBuilder {
    affected_users: Option<i64>,
    assignee_user_id: Option<String>,
    awaiting_input_since: Option<f64>,
    category: Option<QueryProblemsGetByIdResponseCategory>,
    category_tainted_at: Option<f64>,
    compact_title: Option<String>,
    description: Option<String>,
    description_tainted_at: Option<f64>,
    dismiss_reason: Option<String>,
    dismissed_at: Option<f64>,
    dismissed_by: Option<String>,
    dismissed_duplicate_of_id: Option<String>,
    duplicate_of_id: Option<String>,
    fingerprint: Option<String>,
    first_seen_at: Option<f64>,
    has_live_findings: Option<bool>,
    icon: Option<String>,
    id: Option<String>,
    investigation_outcome: Option<String>,
    last_error: Option<String>,
    last_seen_at: Option<f64>,
    next_recovery_check_at: Option<f64>,
    occurrences: Option<i64>,
    organization_id: Option<String>,
    priority: Option<QueryProblemsGetByIdResponsePriority>,
    priority_floor: Option<QueryProblemsGetByIdResponsePriorityFloor>,
    priority_floor_basis: Option<QueryProblemsGetByIdResponsePriorityFloorBasis>,
    priority_tainted_at: Option<f64>,
    regression_probability: Option<f64>,
    resolved_at: Option<f64>,
    sequence: Option<i64>,
    service_id: Option<String>,
    status: Option<QueryProblemsGetByIdResponseStatus>,
    surface_id: Option<String>,
    title: Option<String>,
    title_tainted_at: Option<f64>,
    created_at: Option<f64>,
    deleted_at: Option<f64>,
    updated_at: Option<f64>,
    surface: Option<QueryProblemsGetByIdResponseSurface>,
    assignee: Option<QueryProblemsGetByIdResponseAssignee>,
    timeline_events: Option<Vec<QueryProblemsGetByIdResponseTimelineEventsItem>>,
}

impl QueryProblemsGetByIdResponseBuilder {
    pub fn affected_users(mut self, value: i64) -> Self {
        self.affected_users = Some(value);
        self
    }

    pub fn assignee_user_id(mut self, value: impl Into<String>) -> Self {
        self.assignee_user_id = Some(value.into());
        self
    }

    pub fn awaiting_input_since(mut self, value: f64) -> Self {
        self.awaiting_input_since = Some(value);
        self
    }

    pub fn category(mut self, value: QueryProblemsGetByIdResponseCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn category_tainted_at(mut self, value: f64) -> Self {
        self.category_tainted_at = Some(value);
        self
    }

    pub fn compact_title(mut self, value: impl Into<String>) -> Self {
        self.compact_title = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn description_tainted_at(mut self, value: f64) -> Self {
        self.description_tainted_at = Some(value);
        self
    }

    pub fn dismiss_reason(mut self, value: impl Into<String>) -> Self {
        self.dismiss_reason = Some(value.into());
        self
    }

    pub fn dismissed_at(mut self, value: f64) -> Self {
        self.dismissed_at = Some(value);
        self
    }

    pub fn dismissed_by(mut self, value: impl Into<String>) -> Self {
        self.dismissed_by = Some(value.into());
        self
    }

    pub fn dismissed_duplicate_of_id(mut self, value: impl Into<String>) -> Self {
        self.dismissed_duplicate_of_id = Some(value.into());
        self
    }

    pub fn duplicate_of_id(mut self, value: impl Into<String>) -> Self {
        self.duplicate_of_id = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn first_seen_at(mut self, value: f64) -> Self {
        self.first_seen_at = Some(value);
        self
    }

    pub fn has_live_findings(mut self, value: bool) -> Self {
        self.has_live_findings = Some(value);
        self
    }

    pub fn icon(mut self, value: impl Into<String>) -> Self {
        self.icon = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn investigation_outcome(mut self, value: impl Into<String>) -> Self {
        self.investigation_outcome = Some(value.into());
        self
    }

    pub fn last_error(mut self, value: impl Into<String>) -> Self {
        self.last_error = Some(value.into());
        self
    }

    pub fn last_seen_at(mut self, value: f64) -> Self {
        self.last_seen_at = Some(value);
        self
    }

    pub fn next_recovery_check_at(mut self, value: f64) -> Self {
        self.next_recovery_check_at = Some(value);
        self
    }

    pub fn occurrences(mut self, value: i64) -> Self {
        self.occurrences = Some(value);
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn priority(mut self, value: QueryProblemsGetByIdResponsePriority) -> Self {
        self.priority = Some(value);
        self
    }

    pub fn priority_floor(mut self, value: QueryProblemsGetByIdResponsePriorityFloor) -> Self {
        self.priority_floor = Some(value);
        self
    }

    pub fn priority_floor_basis(mut self, value: QueryProblemsGetByIdResponsePriorityFloorBasis) -> Self {
        self.priority_floor_basis = Some(value);
        self
    }

    pub fn priority_tainted_at(mut self, value: f64) -> Self {
        self.priority_tainted_at = Some(value);
        self
    }

    pub fn regression_probability(mut self, value: f64) -> Self {
        self.regression_probability = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: f64) -> Self {
        self.resolved_at = Some(value);
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn service_id(mut self, value: impl Into<String>) -> Self {
        self.service_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: QueryProblemsGetByIdResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn surface_id(mut self, value: impl Into<String>) -> Self {
        self.surface_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn title_tainted_at(mut self, value: f64) -> Self {
        self.title_tainted_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deleted_at(mut self, value: f64) -> Self {
        self.deleted_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: f64) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn surface(mut self, value: QueryProblemsGetByIdResponseSurface) -> Self {
        self.surface = Some(value);
        self
    }

    pub fn assignee(mut self, value: QueryProblemsGetByIdResponseAssignee) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn timeline_events(mut self, value: Vec<QueryProblemsGetByIdResponseTimelineEventsItem>) -> Self {
        self.timeline_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsGetByIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fingerprint`](QueryProblemsGetByIdResponseBuilder::fingerprint)
    /// - [`first_seen_at`](QueryProblemsGetByIdResponseBuilder::first_seen_at)
    /// - [`id`](QueryProblemsGetByIdResponseBuilder::id)
    /// - [`last_seen_at`](QueryProblemsGetByIdResponseBuilder::last_seen_at)
    /// - [`organization_id`](QueryProblemsGetByIdResponseBuilder::organization_id)
    /// - [`status`](QueryProblemsGetByIdResponseBuilder::status)
    /// - [`created_at`](QueryProblemsGetByIdResponseBuilder::created_at)
    /// - [`updated_at`](QueryProblemsGetByIdResponseBuilder::updated_at)
    /// - [`timeline_events`](QueryProblemsGetByIdResponseBuilder::timeline_events)
    pub fn build(self) -> Result<QueryProblemsGetByIdResponse, BuildError> {
        Ok(QueryProblemsGetByIdResponse {
            affected_users: self.affected_users,
            assignee_user_id: self.assignee_user_id,
            awaiting_input_since: self.awaiting_input_since,
            category: self.category,
            category_tainted_at: self.category_tainted_at,
            compact_title: self.compact_title,
            description: self.description,
            description_tainted_at: self.description_tainted_at,
            dismiss_reason: self.dismiss_reason,
            dismissed_at: self.dismissed_at,
            dismissed_by: self.dismissed_by,
            dismissed_duplicate_of_id: self.dismissed_duplicate_of_id,
            duplicate_of_id: self.duplicate_of_id,
            fingerprint: self.fingerprint.ok_or_else(|| BuildError::missing_field("fingerprint"))?,
            first_seen_at: self.first_seen_at.ok_or_else(|| BuildError::missing_field("first_seen_at"))?,
            has_live_findings: self.has_live_findings,
            icon: self.icon,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            investigation_outcome: self.investigation_outcome,
            last_error: self.last_error,
            last_seen_at: self.last_seen_at.ok_or_else(|| BuildError::missing_field("last_seen_at"))?,
            next_recovery_check_at: self.next_recovery_check_at,
            occurrences: self.occurrences,
            organization_id: self.organization_id.ok_or_else(|| BuildError::missing_field("organization_id"))?,
            priority: self.priority,
            priority_floor: self.priority_floor,
            priority_floor_basis: self.priority_floor_basis,
            priority_tainted_at: self.priority_tainted_at,
            regression_probability: self.regression_probability,
            resolved_at: self.resolved_at,
            sequence: self.sequence,
            service_id: self.service_id,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            surface_id: self.surface_id,
            title: self.title,
            title_tainted_at: self.title_tainted_at,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            deleted_at: self.deleted_at,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            surface: self.surface,
            assignee: self.assignee,
            timeline_events: self.timeline_events.ok_or_else(|| BuildError::missing_field("timeline_events"))?,
        })
    }
}
