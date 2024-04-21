use crate::PdfElement;

static HEXA_ALPHABET: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

pub struct PdfString {
    text: String,
    encoding: PdfStringEncoding,
}

pub enum PdfStringEncoding {
    Literal,
    Hexadecimal,
}

impl PdfElement for PdfString {
    fn print(&self) -> Vec<u8> {
        let mut formatted_text: Vec<u8> = Vec::new();

        match self.encoding {
            PdfStringEncoding::Literal => {
                formatted_text.extend([b"(", self.text.as_bytes(), b")"].concat());
                formatted_text
            }
            PdfStringEncoding::Hexadecimal => {
                formatted_text.extend(b"<");
                formatted_text.extend(bytes_to_hexadecimal(self.text.as_bytes()));
                formatted_text.extend(b">");
                formatted_text
            }
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl PdfString {
    pub fn new(text: String, encoding: PdfStringEncoding) -> Self {
        PdfString { text, encoding }
    }
}

fn bytes_to_hexadecimal(text: &[u8]) -> Vec<u8> {
    let mut formatted_characters: Vec<u8> = Vec::new();

    for character in text {
        let higher = character >> 4u8;
        let lower = character & 0x0Fu8;

        formatted_characters.extend([
            HEXA_ALPHABET[usize::from(higher)],
            HEXA_ALPHABET[usize::from(lower)],
        ]);
    }

    formatted_characters
}
