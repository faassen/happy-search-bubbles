use iri_string::types::{IriAbsoluteString, IriRelativeString};

#[derive(Debug)]
pub struct Indexable {
    uri: IriAbsoluteString,
    scope: Scope,
}

#[derive(Debug)]
pub enum Scope {
    Site, // all linked pages on given URL, as long as it's the same site
    Path, // only the page at the given path and anything linked under that path
    Page, // only this page, nothing else
}

#[derive(Debug)]
pub struct Topic {
    pub title: String,
    pub descriptions: Vec<TopicDescription>,
}

#[derive(Debug)]
pub enum TopicDescription {
    Wikipedia(IriAbsoluteString),
    WikiData(IriAbsoluteString),
}

#[derive(Debug)]
pub struct Bubble {
    pub topic: Topic,
    pub indexables: Vec<Indexable>,
    pub not_indexables: Vec<Indexable>,
}
