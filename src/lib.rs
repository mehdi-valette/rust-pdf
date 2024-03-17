mod objects;

pub use objects::{Dictionary, Document, Header, Name, Reference, Trailer};

pub trait PdfElement {
    fn print(&self) -> Vec<u8>;
}
