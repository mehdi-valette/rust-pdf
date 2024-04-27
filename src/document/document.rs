use std::collections::BTreeMap;

use crate::document::Header;
use crate::objects::{
    print_trailer, Dictionary, IndirectObject, Number, PdfArray, PdfName, Reference,
};
use crate::PdfElement;

use super::reference_table;

pub struct Document {
    pub header: Header,
    pub body: BTreeMap<Reference, IndirectObject>,
    object_count: u32,
    catalog: Reference,
    page_root: Reference,
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

    pub fn get_catalog_reference(&self) -> Reference {
        self.catalog
            .as_any()
            .downcast_ref::<Reference>()
            .expect("cannot get the catalog")
            .clone()
    }

    pub fn get_page_root_reference(&self) -> Reference {
        self.page_root
            .as_any()
            .downcast_ref::<Reference>()
            .expect("cannot get page root")
            .clone()
    }

    pub fn add_page(&mut self, page: Dictionary) {
        let page_reference = self.push(Box::new(page)).get_reference();

        let page_root = self
            .body
            .get_mut(&self.page_root)
            .expect("page root is not found");

        let page_root_dictionary = page_root
            .get_content_mut()
            .as_any_mut()
            .downcast_mut::<Dictionary>()
            .expect("page root doesn't have a dictionary");

        page_root_dictionary
            .get_mut("Kids")
            .expect("page root dictionary has no entry \"Kids\"")
            .as_any_mut()
            .downcast_mut::<PdfArray>()
            .expect("entry \"Kids\" is not an array")
            .add(page_reference);

        page_root_dictionary
            .get_mut("Count")
            .expect("no entry \"Count\"")
            .as_any_mut()
            .downcast_mut::<Number>()
            .expect("\"Count\" is not a number")
            .incr();
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
            page_root: Reference::new(),
        };

        let mut page_size = PdfArray::new();
        page_size
            .add(Number::new(0f32))
            .add(Number::new(0f32))
            .add(Number::new(595f32))
            .add(Number::new(792f32));

        let mut page_tree = Dictionary::new();
        page_tree
            .insert("Type", PdfName::new("Pages"))
            .insert("Kids", PdfArray::new())
            .insert("Count", Number::new(0f32))
            .insert("MediaBox", page_size);

        let page_tree_object = document.push(Box::new(page_tree)).get_reference();

        let mut catalog = Dictionary::new();
        catalog
            .insert("Type", PdfName::new("Catalog"))
            .insert("Pages", page_tree_object.clone());

        let catalog_object = document.push(Box::new(catalog));
        document.catalog = catalog_object.get_reference();
        document.page_root = page_tree_object;

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

        pdf.extend(reference_table::print(
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
