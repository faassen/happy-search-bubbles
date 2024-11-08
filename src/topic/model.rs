use iri_string::types::IriAbsoluteString;

#[derive(Debug, PartialEq, Eq)]
pub struct Topic {
    pub(super) descriptions: Vec<TopicDescription>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TopicDescription {
    Wikipedia(IriAbsoluteString),
    Wikidata(IriAbsoluteString),
}

impl Topic {
    pub fn descriptions(&self) -> &[TopicDescription] {
        &self.descriptions
    }
}
