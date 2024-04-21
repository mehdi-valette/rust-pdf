use crate::{PdfArray, PdfElement, Reference};

pub struct IndirectObject {
    object_number: u32,
    generation_number: u32,
    content: Box<dyn PdfElement>,
    offset: u32,
}

impl IndirectObject {
    pub fn new() -> IndirectObject {
        IndirectObject {
            object_number: 0,
            generation_number: 0,
            content: Box::new(PdfArray::new()),
            offset: 0,
        }
    }

    pub fn make(object_number: u32, generation_number: u32, content: Box<dyn PdfElement>) -> Self {
        IndirectObject {
            object_number,
            generation_number,
            content,
            offset: 0,
        }
    }

    pub fn get_reference(&self) -> Reference {
        Reference::make(self)
    }

    pub fn get_object_number(&self) -> &u32 {
        &self.object_number
    }

    pub fn get_generation_number(&self) -> &u32 {
        &self.generation_number
    }

    pub fn get_content(&self) -> &Box<dyn PdfElement> {
        &self.content
    }

    pub fn get_content_mut(&mut self) -> &mut Box<dyn PdfElement> {
        &mut self.content
    }

    pub fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }

    pub fn get_offset(&self) -> &u32 {
        &self.offset
    }
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
