use iri_string::types::IriAbsoluteString;

#[derive(Debug)]
pub struct Topic {
    pub(super) descriptions: Vec<TopicDescription>,
}

#[derive(Debug)]
pub enum TopicDescription {
    Wikipedia(IriAbsoluteString),
    WikiData(IriAbsoluteString),
}
