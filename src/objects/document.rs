use std::collections::BTreeMap;

use crate::reference_table::print as print_reference_table;
use crate::{
    print_trailer, Dictionary, Header, IndirectObject, Number, PdfArray, PdfElement, PdfName,
    Reference,
};

pub struct Document {
    pub header: Header,
    pub body: BTreeMap<Reference, IndirectObject>,
    object_count: u32,
    catalog: Reference,
}

impl Document {
    pub fn push(&mut self, object: Box<dyn PdfElement>) -> &IndirectObject {
        self.object_count += 1;

        let object = IndirectObject::make(self.object_count, 0, object);

        let object_reference = object.get_reference();

        self.body.insert(object_reference.clone(), object);

        match self.get_object(object_reference) {
            None => panic!("Cannot find the object that was just inserted"),
            Some(object) => object,
        }
    }

    pub fn get_object(&self, id: Reference) -> Option<&IndirectObject> {
        self.body.get(&id)
    }

    pub fn get_object_mut(&mut self, id: Reference) -> Option<&mut IndirectObject> {
        self.body.get_mut(&id)
    }

    pub fn get_catalog(&self) -> Reference {
        match self.catalog.as_any().downcast_ref::<Reference>() {
            None => panic!("Cannot get the dictionary for the catalog"),
            Some(dictionary) => dictionary.clone(),
        }
    }

    pub fn new() -> Self {
        let header: Header = Header {
            pdf_version: "2.0".into(),
        };

        let body: BTreeMap<Reference, IndirectObject> = BTreeMap::new();

        let mut document = Document {
            header,
            body,
            object_count: 0,
            catalog: Reference::new(),
        };

        let mut page_tree = Dictionary::new();
        page_tree
            .insert("Type", PdfName::new("Pages"))
            .insert("Kids", PdfArray::new())
            .insert("Count", Number::new(0f32));

        let page_tree_object = document.push(Box::new(page_tree));

        let mut catalog = Dictionary::new();
        catalog
            .insert("Type", PdfName::new("Catalog"))
            .insert("Pages", Reference::make(page_tree_object));

        let catalog_object = document.push(Box::new(catalog));
        document.catalog = catalog_object.get_reference();

        document
    }

    pub fn print(&mut self) -> Vec<u8> {
        let mut pdf: Vec<u8> = Vec::new();

        pdf.extend(self.header.print());
        pdf.extend(b"\n\n");

        for (_, object) in self.body.iter_mut() {
            object.set_offset(pdf.len() as u32);

            pdf.extend(object.print());
            pdf.extend(b"\n\n");
        }

        let xref_begining = pdf.len() as u32;

        pdf.extend(print_reference_table(
            &self.body.iter().map(|(_, value)| value).collect(),
        ));

        pdf.extend(b"\n");

        pdf.extend(print_trailer(
            &xref_begining,
            &(self.object_count + 1),
            &self.catalog,
        ));

        pdf
    }
}
