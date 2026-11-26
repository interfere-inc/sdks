pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentTenConfirmationIdObservedWindow {
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub from: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub to: i64,
}

impl TimelineEventContentTenConfirmationIdObservedWindow {
    pub fn builder() -> TimelineEventContentTenConfirmationIdObservedWindowBuilder {
        <TimelineEventContentTenConfirmationIdObservedWindowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentTenConfirmationIdObservedWindowBuilder {
    from: Option<i64>,
    to: Option<i64>,
}

impl TimelineEventContentTenConfirmationIdObservedWindowBuilder {
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentTenConfirmationIdObservedWindow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](TimelineEventContentTenConfirmationIdObservedWindowBuilder::from)
    /// - [`to`](TimelineEventContentTenConfirmationIdObservedWindowBuilder::to)
    pub fn build(self) -> Result<TimelineEventContentTenConfirmationIdObservedWindow, BuildError> {
        Ok(TimelineEventContentTenConfirmationIdObservedWindow {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
