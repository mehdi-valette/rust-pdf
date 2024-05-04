use crate::PdfElement;

static HEXA_ALPHABET: [u8; 16] = [
    b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'A', b'B', b'C', b'D', b'E', b'F',
];

#[derive(Debug, PartialEq)]
pub enum PdfStringEncoding {
    Literal,
    Hexadecimal,
}

#[derive(Debug, PartialEq)]
pub struct PdfString {
    text: Vec<u8>,
    encoding: PdfStringEncoding,
}

impl PdfString {
    pub fn from_string(text: String, encoding: PdfStringEncoding) -> Self {
        PdfString {
            text: text.into(),
            encoding,
        }
    }

    pub fn from_vec(text: Vec<u8>, encoding: PdfStringEncoding) -> Self {
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

impl PdfElement for PdfString {
    fn print(&self) -> Vec<u8> {
        let mut formatted_text: Vec<u8> = Vec::new();

        match self.encoding {
            PdfStringEncoding::Literal => {
                formatted_text.extend([b"(", self.text.as_slice(), b")"].concat());
                formatted_text
            }
            PdfStringEncoding::Hexadecimal => {
                formatted_text.extend(b"<");
                formatted_text.extend(bytes_to_hexadecimal(self.text.as_slice()));
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_string() {
        let test_data = [
            (
                ("helloé!$".to_string(), PdfStringEncoding::Literal),
                PdfString {
                    text: "helloé!$".as_bytes().to_vec(),
                    encoding: PdfStringEncoding::Literal,
                },
            ),
            (
                ("helloé!$".to_string(), PdfStringEncoding::Hexadecimal),
                PdfString {
                    text: "helloé!$".as_bytes().to_vec(),
                    encoding: PdfStringEncoding::Hexadecimal,
                },
            ),
        ];

        for test in test_data {
            assert_eq!(PdfString::from_string(test.0 .0, test.0 .1), test.1);
        }
    }

    #[test]
    fn test_from_vec() {
        let test_data = [
            (
                ("helloé!$".as_bytes().to_vec(), PdfStringEncoding::Literal),
                PdfString {
                    text: "helloé!$".as_bytes().to_vec(),
                    encoding: PdfStringEncoding::Literal,
                },
            ),
            (
                (
                    "helloé!$".as_bytes().to_vec(),
                    PdfStringEncoding::Hexadecimal,
                ),
                PdfString {
                    text: "helloé!$".as_bytes().to_vec(),
                    encoding: PdfStringEncoding::Hexadecimal,
                },
            ),
        ];

        for test in test_data {
            assert_eq!(PdfString::from_vec(test.0 .0, test.0 .1), test.1);
        }
    }

    #[test]
    fn test_print() {
        let test_data = [
            (
                ("Helloé!$".to_string(), PdfStringEncoding::Literal),
                "(Helloé!$)".as_bytes().to_vec(),
            ),
            (
                ("Helloé!$".to_string(), PdfStringEncoding::Hexadecimal),
                "<48656C6C6FC3A92124>".as_bytes().to_vec(),
            ),
        ];

        for test in test_data {
            let result = PdfString::from_string(test.0 .0, test.0 .1).print();

            assert_eq!(result, test.1)
        }
    }

    #[test]
    fn test_bytes_to_hexadecimal() {
        let test_data = [(&[0x24, 0xF8, 0xFF, 0x00], "24F8FF00".as_bytes().to_vec())];

        for test in test_data {
            assert_eq!(bytes_to_hexadecimal(test.0), test.1);
        }
    }
}
