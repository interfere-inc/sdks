pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsSeenUserResponseSourceZero {
    pub name: QueryTenantsSeenUserResponseSourceZeroName,
    pub r#type: QueryTenantsSeenUserResponseSourceZeroType,
}

impl QueryTenantsSeenUserResponseSourceZero {
    pub fn builder() -> QueryTenantsSeenUserResponseSourceZeroBuilder {
        <QueryTenantsSeenUserResponseSourceZeroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsSeenUserResponseSourceZeroBuilder {
    name: Option<QueryTenantsSeenUserResponseSourceZeroName>,
    r#type: Option<QueryTenantsSeenUserResponseSourceZeroType>,
}

impl QueryTenantsSeenUserResponseSourceZeroBuilder {
    pub fn name(mut self, value: QueryTenantsSeenUserResponseSourceZeroName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryTenantsSeenUserResponseSourceZeroType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsSeenUserResponseSourceZero`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsSeenUserResponseSourceZeroBuilder::name)
    /// - [`r#type`](QueryTenantsSeenUserResponseSourceZeroBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsSeenUserResponseSourceZero, BuildError> {
        Ok(QueryTenantsSeenUserResponseSourceZero {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
