pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsSeenUserResponseSourceOne {
    pub name: QueryTenantsSeenUserResponseSourceOneName,
    pub r#type: QueryTenantsSeenUserResponseSourceOneType,
}

impl QueryTenantsSeenUserResponseSourceOne {
    pub fn builder() -> QueryTenantsSeenUserResponseSourceOneBuilder {
        <QueryTenantsSeenUserResponseSourceOneBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsSeenUserResponseSourceOneBuilder {
    name: Option<QueryTenantsSeenUserResponseSourceOneName>,
    r#type: Option<QueryTenantsSeenUserResponseSourceOneType>,
}

impl QueryTenantsSeenUserResponseSourceOneBuilder {
    pub fn name(mut self, value: QueryTenantsSeenUserResponseSourceOneName) -> Self {
        self.name = Some(value);
        self
    }

    pub fn r#type(mut self, value: QueryTenantsSeenUserResponseSourceOneType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsSeenUserResponseSourceOne`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsSeenUserResponseSourceOneBuilder::name)
    /// - [`r#type`](QueryTenantsSeenUserResponseSourceOneBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsSeenUserResponseSourceOne, BuildError> {
        Ok(QueryTenantsSeenUserResponseSourceOne {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
