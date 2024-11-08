use iri_string::types::IriAbsoluteString;

#[derive(Debug, PartialEq, Eq)]
pub struct Indexable {
    pub(super) uri: IriAbsoluteString,
    pub(super) scope: Scope,
    pub(super) label: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    Site, // all linked pages on given URL, as long as it's the same site
    Path, // only the page at the given path and anything linked under that path
    Page, // only this page, nothing else
          // TODO: more complex identifiers, like "all blog entries which have topic X". This r
          // probably requires some kind of trait where site-specific knowledge can
          // be described
}

impl Indexable {
    pub fn new(uri: IriAbsoluteString, scope: Scope, label: String) -> Self {
        Self { uri, scope, label }
    }
}
