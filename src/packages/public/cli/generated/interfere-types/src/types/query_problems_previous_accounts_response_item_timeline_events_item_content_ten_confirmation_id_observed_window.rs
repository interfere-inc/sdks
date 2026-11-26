pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindow {
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub from: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub to: i64,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindow {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindowBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindowBuilder {
    from: Option<i64>,
    to: Option<i64>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindowBuilder {
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindowBuilder::from)
    /// - [`to`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindowBuilder::to)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindow, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentTenConfirmationIdObservedWindow {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
