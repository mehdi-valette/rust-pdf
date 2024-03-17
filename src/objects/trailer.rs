use crate::{Dictionary, PdfElement};

pub struct Trailer {
    pub dictionary: Dictionary,
}

impl PdfElement for Trailer {
    fn print(&self) -> Vec<u8> {
        [b"trailer\n", self.dictionary.print().as_slice()].concat()
    }
}
