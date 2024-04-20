use crate::PdfElement;

pub struct Number {
    number: f32,
}

impl Number {
    pub fn new(number: f32) -> Number {
        Number { number }
    }
}

impl PdfElement for Number {
    fn print(&self) -> Vec<u8> {
        format!("{}", self.number).into()
    }
}
