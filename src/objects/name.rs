use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

use crate::PdfElement;

pub struct PdfName {
    name: Vec<u8>,
}

impl PartialOrd for PdfName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for PdfName {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl PdfElement for PdfName {
    fn print(&self) -> Vec<u8> {
        [b"/", self.name.as_slice()].concat()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PartialEq for PdfName {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for PdfName {}

impl Hash for PdfName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PdfName {
    pub fn new(text: &str) -> Self {
        PdfName { name: text.into() }
    }
}
