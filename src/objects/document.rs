use crate::reference_table::print as print_reference_table;
use crate::{Header, IndirectObject, PdfElement, Trailer};

pub struct Document {
    pub header: Header,
    pub body: Vec<IndirectObject>,
    pub trailer: Trailer,
}

impl Document {
    pub fn print(&mut self) -> Vec<u8> {
        let mut pdf: Vec<u8> = Vec::new();

        pdf.extend(self.header.print());
        pdf.extend(b"\n\n");

        for object in self.body.iter_mut() {
            object.set_offset(pdf.len() as u32);

            pdf.extend(object.print());
            pdf.extend(b"\n\n");
        }

        let xref_begining = pdf.len() as u32;

        pdf.extend(print_reference_table(&self.body));

        pdf.extend(b"\n");

        pdf.extend(self.trailer.print_trailer(&xref_begining));

        pdf
    }
}
