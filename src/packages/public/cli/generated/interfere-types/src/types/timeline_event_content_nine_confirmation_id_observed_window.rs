pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TimelineEventContentNineConfirmationIdObservedWindow {
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub from: i64,
    /// Milliseconds since the Unix epoch in UTC.
    #[serde(default)]
    pub to: i64,
}

impl TimelineEventContentNineConfirmationIdObservedWindow {
    pub fn builder() -> TimelineEventContentNineConfirmationIdObservedWindowBuilder {
        <TimelineEventContentNineConfirmationIdObservedWindowBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TimelineEventContentNineConfirmationIdObservedWindowBuilder {
    from: Option<i64>,
    to: Option<i64>,
}

impl TimelineEventContentNineConfirmationIdObservedWindowBuilder {
    pub fn from(mut self, value: i64) -> Self {
        self.from = Some(value);
        self
    }

    pub fn to(mut self, value: i64) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TimelineEventContentNineConfirmationIdObservedWindow`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from`](TimelineEventContentNineConfirmationIdObservedWindowBuilder::from)
    /// - [`to`](TimelineEventContentNineConfirmationIdObservedWindowBuilder::to)
    pub fn build(self) -> Result<TimelineEventContentNineConfirmationIdObservedWindow, BuildError> {
        Ok(TimelineEventContentNineConfirmationIdObservedWindow {
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
