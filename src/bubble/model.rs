use iri_string::types::IriAbsoluteString;

use crate::topic::Topic;

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
pub struct Bubble {
    pub(super) title: String,
    pub(super) topic: Topic,
    pub(super) indexables: Vec<Indexable>,
    pub(super) not_indexables: Vec<Indexable>,
}
