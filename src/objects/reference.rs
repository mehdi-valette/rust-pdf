use crate::PdfElement;

pub struct Reference {
    pub object_number: u32,
    pub generation_number: u32,
}

impl PdfElement for Reference {
    fn print(&self) -> Vec<u8> {
        format!("{} {} R", self.object_number, self.generation_number)
            .as_bytes()
            .to_vec()
    }
}
