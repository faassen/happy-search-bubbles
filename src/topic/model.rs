use iri_string::types::IriAbsoluteString;

#[derive(Debug, PartialEq, Eq)]
pub struct Topic {
    pub(super) references: Vec<TopicReference>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopicCategory {
    Wikipedia,
    Wikidata,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TopicReference {
    pub(super) uri: IriAbsoluteString,
    pub(super) label: String,
    pub(super) category: TopicCategory,
}

impl Topic {
    pub fn references(&self) -> &[TopicReference] {
        &self.references
    }
}

impl TopicReference {
    pub fn new(uri: IriAbsoluteString, label: String, category: TopicCategory) -> Self {
        Self {
            uri,
            label,
            category,
        }
    }
}
