use iri_string::types::{IriAbsoluteString, IriRelativeString};

pub struct Indexable {
    uri: IriAbsoluteString,
    scope: Scope,
}

pub enum Scope {
    Site, // all linked pages on given URL, as long as it's the same site
    Path, // only the page at the given path and anything linked under that path
    Page, // only this page, nothing else
}

pub struct Topic {
    title: String,
    descriptions: Vec<TopicDescription>,
}

pub enum TopicDescription {
    Wikipedia(IriAbsoluteString),
    WikiData(IriAbsoluteString),
}

pub struct Bubble {
    topic: Topic,
    indexables: Vec<Indexable>,
    not_indexables: Vec<Indexable>,
}
