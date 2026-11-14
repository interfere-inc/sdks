pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeType,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree {
    pub fn builder() -> QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeBuilder {
        <QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeType>,
}

impl QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeBuilder::name)
    /// - [`r#type`](QueryTenantsGetPageQueryResponseItemConsumersItemSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree, BuildError> {
        Ok(QueryTenantsGetPageQueryResponseItemConsumersItemSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
