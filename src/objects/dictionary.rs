use std::collections::HashMap;

use crate::{Name, PdfElement};

pub struct Dictionary {
    pub dict: HashMap<Name, Box<dyn PdfElement>>,
}

impl PdfElement for Dictionary {
    fn print(&self) -> Vec<u8> {
        let mut entries: Vec<u8> = Vec::new();
        entries.extend(b"<<");

        for (key, value) in &(self.dict) {
            entries.extend([b"\n", key.print().as_slice(), b" "].concat());
            entries.extend(value.print());
        }

        entries.extend(b"\n>>");

        entries
    }
}
