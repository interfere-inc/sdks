pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsSeenUserResponseSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: QueryTenantsSeenUserResponseSourceThreeType,
}

impl QueryTenantsSeenUserResponseSourceThree {
    pub fn builder() -> QueryTenantsSeenUserResponseSourceThreeBuilder {
        <QueryTenantsSeenUserResponseSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsSeenUserResponseSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<QueryTenantsSeenUserResponseSourceThreeType>,
}

impl QueryTenantsSeenUserResponseSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: QueryTenantsSeenUserResponseSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsSeenUserResponseSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsSeenUserResponseSourceThreeBuilder::name)
    /// - [`r#type`](QueryTenantsSeenUserResponseSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsSeenUserResponseSourceThree, BuildError> {
        Ok(QueryTenantsSeenUserResponseSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
