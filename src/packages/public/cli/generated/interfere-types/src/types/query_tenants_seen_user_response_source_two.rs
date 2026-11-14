pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsSeenUserResponseSourceTwo {
    pub name: QueryTenantsSeenUserResponseSourceTwoName,
    pub r#type: QueryTenantsSeenUserResponseSourceTwoType,
}

impl QueryTenantsSeenUserResponseSourceTwo {
    pub fn builder() -> QueryTenantsSeenUserResponseSourceTwoBuilder {
        <QueryTenantsSeenUserResponseSourceTwoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsSeenUserResponseSourceTwoBuilder {
    name: Option<QueryTenantsSeenUserResponseSourceTwoName>,
    r#type: Option<QueryTenantsSeenUserResponseSourceTwoType>,
}

impl QueryTenantsSeenUserResponseSourceTwoBuilder {
    pub fn name(mut self, value: QueryTenantsSeenUserResponseSourceTwoName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryTenantsSeenUserResponseSourceTwoType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsSeenUserResponseSourceTwo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsSeenUserResponseSourceTwoBuilder::name)
    /// - [`r#type`](QueryTenantsSeenUserResponseSourceTwoBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsSeenUserResponseSourceTwo, BuildError> {
        Ok(QueryTenantsSeenUserResponseSourceTwo {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
