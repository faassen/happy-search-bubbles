use iri_string::types::IriAbsoluteString;

#[derive(Debug, PartialEq, Eq)]
pub struct Topic {
    pub(super) descriptions: Vec<TopicDescription>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TopicDescription {
    Wikipedia(IriAbsoluteString),
    WikiData(IriAbsoluteString),
}
