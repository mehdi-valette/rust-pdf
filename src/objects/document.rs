use crate::{Header, PdfElement, Trailer};

pub struct Document {
    pub header: Header,
    pub body: Vec<Box<dyn PdfElement>>,
    pub trailer: Trailer,
}

impl PdfElement for Document {
    fn print(&self) -> Vec<u8> {
        let mut pdf: Vec<u8> = Vec::new();

        pdf.extend(self.header.print());
        pdf.extend(b"\n\n");

        for object in self.body.iter() {
            pdf.extend(object.print());
            pdf.extend(b"\n\n");
        }

        pdf.extend(self.trailer.print());

        pdf
    }
}
