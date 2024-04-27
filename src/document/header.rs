use crate::PdfElement;

pub struct Header {
    pub pdf_version: Vec<u8>,
}

impl PdfElement for Header {
    fn print(&self) -> Vec<u8> {
        [b"%PDF-", self.pdf_version.as_slice()].concat()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
