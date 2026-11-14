pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryInvitationsForOrgResponse(pub Vec<QueryInvitationsForOrgResponseItem>);