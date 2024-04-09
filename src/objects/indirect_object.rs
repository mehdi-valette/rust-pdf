use crate::PdfElement;

pub struct IndirectObject {
    object_number: u32,
    generation_number: u32,
    content: Box<dyn PdfElement>,
}

impl PdfElement for IndirectObject {
    fn print(&self) -> Vec<u8> {
        let mut object: Vec<u8> = Vec::new();

        object
            .extend(format!("{} {} obj\n", self.object_number, self.generation_number).as_bytes());

        object.extend(self.content.print());

        object.extend(b"\nendobj");

        object
    }
}

impl IndirectObject {
    pub fn new(object_number: u32, generation_number: u32, content: Box<dyn PdfElement>) -> Self {
        IndirectObject {
            object_number,
            generation_number,
            content,
        }
    }
}
