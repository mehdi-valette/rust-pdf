use crate::PdfElement;

pub struct Reference {
    pub object_number: u32,
    pub genration_number: u32,
}

impl PdfElement for Reference {
    fn print(&self) -> Vec<u8> {
        format!("{} {} R", self.object_number, self.genration_number)
            .as_bytes()
            .to_vec()
    }
}
