use crate::PdfElement;

pub struct Reference {
    pub identifier: u32,
    pub update: u32,
}

impl PdfElement for Reference {
    fn print(&self) -> Vec<u8> {
        format!("{} {} R", self.identifier, self.update)
            .as_bytes()
            .to_vec()
    }
}
