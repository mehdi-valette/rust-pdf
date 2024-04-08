mod objects;

pub use objects::{
    Dictionary, Document, Header, Name, PdfBoolean, PdfString, PdfStringEncoding, Reference,
    Trailer,
};

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
}
