pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDiagram>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc>,
    #[serde(rename = "compactText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compact_text: Option<String>,
    #[serde(rename = "seenAgain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seen_again: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartSeenAgain>,
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub version: String,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart {
    pub fn builder() -> QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartBuilder {
        <QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartBuilder {
    chart: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChart>,
    diagram: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDiagram>,
    doc: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc>,
    compact_text: Option<String>,
    seen_again: Option<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartSeenAgain>,
    text: Option<String>,
    version: Option<String>,
}

impl QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartBuilder {
    pub fn chart(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartChart) -> Self {
        self.chart = Some(value);
        self
    }

    pub fn diagram(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDiagram) -> Self {
        self.diagram = Some(value);
        self
    }

    pub fn doc(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartDoc) -> Self {
        self.doc = Some(value);
        self
    }

    pub fn compact_text(mut self, value: impl Into<String>) -> Self {
        self.compact_text = Some(value.into());
        self
    }

    pub fn seen_again(mut self, value: QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartSeenAgain) -> Self {
        self.seen_again = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartBuilder::text)
    /// - [`version`](QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChartBuilder::version)
    pub fn build(self) -> Result<QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart, BuildError> {
        Ok(QueryProblemsPreviousAccountsResponseItemTimelineEventsItemContentChart {
            chart: self.chart,
            diagram: self.diagram,
            doc: self.doc,
            compact_text: self.compact_text,
            seen_again: self.seen_again,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
