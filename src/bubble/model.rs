use iri_string::types::IriAbsoluteString;

use crate::{indexable::Indexable, topic::Topic};

#[derive(Debug, PartialEq, Eq)]
pub struct Bubble {
    pub(super) title: String,
    pub(super) topic: Topic,
    pub(super) expands: Vec<BubbleReference>,
    pub(super) indexables: Vec<Indexable>,
    pub(super) excludes: Vec<Indexable>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BubbleReference {
    pub(super) uri: IriAbsoluteString,
    pub(super) label: String,
}
