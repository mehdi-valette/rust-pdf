use crate::reference_table::print as print_reference_table;
use crate::{print_trailer, Header, IndirectObject, PdfElement};

pub struct Document {
    pub header: Header,
    pub body: Vec<IndirectObject>,
    object_count: u32,
}

impl Document {
    pub fn push(&mut self, object: Box<dyn PdfElement>) -> &u32 {
        self.object_count += 1;

        self.body
            .push(IndirectObject::new(self.object_count, 0, object));

        &self.object_count
    }

    pub fn new() -> Self {
        let header = Header {
            pdf_version: "2.0".into(),
        };

        let body: Vec<IndirectObject> = Vec::new();

        Document {
            header,
            body,
            object_count: 0,
        }
    }

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

        pdf.extend(print_trailer(&xref_begining, &self.object_count));

        pdf
    }
}
