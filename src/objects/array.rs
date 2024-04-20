use crate::PdfElement;

pub struct PdfArray {
    values: Vec<Box<dyn PdfElement>>,
}

impl PdfArray {
    pub fn new() -> Self {
        PdfArray { values: Vec::new() }
    }

    pub fn make(values: Vec<Box<dyn PdfElement>>) -> Self {
        PdfArray { values }
    }
}

impl PdfElement for PdfArray {
    fn print(&self) -> Vec<u8> {
        let mut text: Vec<u8> = Vec::new();

        text.extend(b"[");

        for (index, value) in self.values.iter().enumerate() {
            if index != 0 {
                text.extend(b",");
            }

            text.extend(value.print());
        }

        text.extend(b"]");

        text
    }
}
