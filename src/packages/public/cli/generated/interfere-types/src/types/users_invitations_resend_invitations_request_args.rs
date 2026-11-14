pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendInvitationsRequestArgs {
    #[serde(rename = "clerkInvitationId")]
    #[serde(default)]
    pub clerk_invitation_id: String,
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(default)]
    pub id: String,
    pub role: ResendInvitationsRequestArgsRole,
}

impl ResendInvitationsRequestArgs {
    pub fn builder() -> ResendInvitationsRequestArgsBuilder {
        <ResendInvitationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInvitationsRequestArgsBuilder {
    clerk_invitation_id: Option<String>,
    email_address: Option<String>,
    id: Option<String>,
    role: Option<ResendInvitationsRequestArgsRole>,
}

impl ResendInvitationsRequestArgsBuilder {
    pub fn clerk_invitation_id(mut self, value: impl Into<String>) -> Self {
        self.clerk_invitation_id = Some(value.into());
        self
    }

    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn role(mut self, value: ResendInvitationsRequestArgsRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendInvitationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`clerk_invitation_id`](ResendInvitationsRequestArgsBuilder::clerk_invitation_id)
    /// - [`email_address`](ResendInvitationsRequestArgsBuilder::email_address)
    /// - [`id`](ResendInvitationsRequestArgsBuilder::id)
    /// - [`role`](ResendInvitationsRequestArgsBuilder::role)
    pub fn build(self) -> Result<ResendInvitationsRequestArgs, BuildError> {
        Ok(ResendInvitationsRequestArgs {
            clerk_invitation_id: self.clerk_invitation_id.ok_or_else(|| BuildError::missing_field("clerk_invitation_id"))?,
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
