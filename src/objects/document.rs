use crate::{Header, PdfElement, Trailer};

pub struct Document {
    pub header: Header,
    pub trailer: Trailer,
}

impl PdfElement for Document {
    fn print(&self) -> Vec<u8> {
        [
            self.header.print().as_slice(),
            b"\n\n",
            self.trailer.print().as_slice(),
        ]
        .concat()
    }
}
