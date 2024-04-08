use crate::PdfElement;

pub struct PdfBoolean {
    value: bool,
}

impl PdfElement for PdfBoolean {
    fn print(&self) -> Vec<u8> {
        if self.value {
            Vec::from(b"true")
        } else {
            Vec::from(b"false")
        }
    }
}

impl PdfBoolean {
    pub fn new(value: bool) -> Self {
        PdfBoolean { value }
    }
}
