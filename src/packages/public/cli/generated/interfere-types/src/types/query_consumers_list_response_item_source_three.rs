pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct QueryConsumersListResponseItemSourceThree {
    #[serde(default)]
    pub name: String,
    pub r#type: QueryConsumersListResponseItemSourceThreeType,
}

impl QueryConsumersListResponseItemSourceThree {
    pub fn builder() -> QueryConsumersListResponseItemSourceThreeBuilder {
        <QueryConsumersListResponseItemSourceThreeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryConsumersListResponseItemSourceThreeBuilder {
    name: Option<String>,
    r#type: Option<QueryConsumersListResponseItemSourceThreeType>,
}

impl QueryConsumersListResponseItemSourceThreeBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: QueryConsumersListResponseItemSourceThreeType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryConsumersListResponseItemSourceThree`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](QueryConsumersListResponseItemSourceThreeBuilder::name)
    /// - [`r#type`](QueryConsumersListResponseItemSourceThreeBuilder::r#type)
    pub fn build(self) -> Result<QueryConsumersListResponseItemSourceThree, BuildError> {
        Ok(QueryConsumersListResponseItemSourceThree {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            r#type: self.r#type.ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
