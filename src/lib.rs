mod objects;

pub use objects::{
    Dictionary, Document, Header, IndirectObject, PdfBoolean, PdfName, PdfString,
    PdfStringEncoding, Reference, Trailer,
};

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
}
