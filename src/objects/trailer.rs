use crate::{Dictionary, PdfElement};

pub struct Trailer {
    pub dictionary: Dictionary,
}

impl Trailer {
    pub fn print_trailer(&self, xref_offset: &u32) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();

        text.extend(b"trailer");
        text.extend(self.dictionary.print().as_slice());
        text.extend(b"\nstartxref\n");
        text.extend(format!("{}\n", xref_offset).as_bytes());
        text.extend(b"%%EOF");

        text
    }
}
