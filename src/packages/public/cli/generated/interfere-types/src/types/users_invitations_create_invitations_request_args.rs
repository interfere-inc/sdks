pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateInvitationsRequestArgs {
    #[serde(rename = "emailAddress")]
    #[serde(default)]
    pub email_address: String,
    #[serde(default)]
    pub id: String,
    pub role: CreateInvitationsRequestArgsRole,
}

impl CreateInvitationsRequestArgs {
    pub fn builder() -> CreateInvitationsRequestArgsBuilder {
        <CreateInvitationsRequestArgsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvitationsRequestArgsBuilder {
    email_address: Option<String>,
    id: Option<String>,
    role: Option<CreateInvitationsRequestArgsRole>,
}

impl CreateInvitationsRequestArgsBuilder {
    pub fn email_address(mut self, value: impl Into<String>) -> Self {
        self.email_address = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn role(mut self, value: CreateInvitationsRequestArgsRole) -> Self {
        self.role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateInvitationsRequestArgs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email_address`](CreateInvitationsRequestArgsBuilder::email_address)
    /// - [`id`](CreateInvitationsRequestArgsBuilder::id)
    /// - [`role`](CreateInvitationsRequestArgsBuilder::role)
    pub fn build(self) -> Result<CreateInvitationsRequestArgs, BuildError> {
        Ok(CreateInvitationsRequestArgs {
            email_address: self.email_address.ok_or_else(|| BuildError::missing_field("email_address"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
        })
    }
}
