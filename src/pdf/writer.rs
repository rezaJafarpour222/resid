use lopdf::{
    Document, Object,
    content::{Content, Operation},
    dictionary,
};

use crate::{
    error::AppError,
    font::{loader::Font, types::ShapedText},
    units::Pt,
};

pub struct PdfWriter {
    pub document: Document,
    pub page_id: lopdf::ObjectId,
    pub operations: Vec<Operation>,
    pub page_height: Pt,
    pub page_width: Pt,
    catalog_id: lopdf::ObjectId,
    pub pages: Vec<(lopdf::ObjectId, Vec<Operation>)>,
    pub font: Option<Font>,
    pub font_installed: bool,
    font_resources_id: Option<lopdf::ObjectId>,
}
impl PdfWriter {
    pub fn new(width: Pt, height: Pt) -> Self {
        let mut document = Document::with_version("1.7");
        let page_id = document.new_object_id();
        let pages_id = document.new_object_id();
        let catalog_id = document.new_object_id();
        document.objects.insert(
            page_id,
            dictionary! {
                "Type"=>"Page",
                "Parent"=>pages_id,
                "MediaBox"=>vec![
                    0.into(),
                    0.into(),
                    width.value().into(),
                    height.value().into()
                ],
            }
            .into(),
        );
        document.objects.insert(
            pages_id,
            dictionary! {
                "Type"=>"Pages",
                "Kids"=>vec![page_id.into()],
                "Count"=>1,
            }
            .into(),
        );

        document.objects.insert(
            catalog_id,
            dictionary! {
                "Type" => "Catalog",
                "Pages" => pages_id,
            }
            .into(),
        );
        document.trailer.set("Root", catalog_id);
        Self {
            document,
            page_id,
            operations: Vec::new(),
            page_height: height,
            page_width: width,
            catalog_id,
            pages: Vec::new(),
            font: None,
            font_installed: false,
            font_resources_id: None,
        }
    }

    pub fn install_font(&mut self, shaped_texts: &[ShapedText]) -> Result<(), AppError> {
        if self.font_installed {
            return Ok(());
        }

        if shaped_texts.is_empty() {
            return Err(AppError::PdfWriter(
                "cannot install font without text".to_string(),
            ));
        }

        let font_file_id = self.embed_font()?;

        let descriptor_id = self.create_font_descriptor(font_file_id)?;

        let cid_to_gid_map = self.create_cid_to_gid_map();

        let cid_font_id = self.create_cid_font(descriptor_id, cid_to_gid_map, shaped_texts)?;

        let to_unicode_id = self.create_to_unicode(shaped_texts)?;

        let type0_id = self.create_type0_font(cid_font_id, to_unicode_id);

        let resources_id = self.document.new_object_id();
        self.font_resources_id = Some(resources_id);

        self.document.objects.insert(
            resources_id,
            dictionary! {
                "Font" => dictionary! {
                    "F1" => type0_id,
                },
            }
            .into(),
        );

        if let Some(Object::Dictionary(page)) = self.document.objects.get_mut(&self.page_id) {
            page.set("Resources", resources_id);
        }

        self.font_installed = true;

        Ok(())
    }

    pub fn new_page(&mut self) {
        let old_id = self.page_id;
        let old_ops = std::mem::take(&mut self.operations);
        self.pages.push((old_id, old_ops));
        let page_id = self.document.new_object_id();
        self.document.objects.insert(page_id, dictionary! {
            "Type" => "Page",
            "MediaBox" => vec![0.into(),0.into(),self.page_width.value().into(),self.page_height.value().into()],
        }.into());
        self.page_id = page_id;
        if let Some(resources) = self.font_resources_id {
            if let Some(Object::Dictionary(page)) = self.document.objects.get_mut(&self.page_id) {
                page.set("Resources", resources);
            }
        }
    }

    fn finalize_pages(&mut self) -> Result<(), std::io::Error> {
        let current = std::mem::take(&mut self.operations);
        self.pages.push((self.page_id, current));
        let pages_id = self.document.new_object_id();
        let mut kids = Vec::new();
        for (page_id, ops) in &self.pages {
            let content = Content {
                operations: ops.clone(),
            };
            let data = content
                .encode()
                .map_err(|e| std::io::Error::other(e.to_string()))?;
            let content_id = self.document.new_object_id();
            self.document.objects.insert(
                content_id,
                Object::Stream(lopdf::Stream::new(dictionary! {}, data)),
            );
            if let Some(Object::Dictionary(page)) = self.document.objects.get_mut(page_id) {
                page.set("Parent", pages_id);
                page.set("Contents", content_id);
                if let Some(resources) = self.font_resources_id {
                    page.set("Resources", resources);
                }
            }
            kids.push((*page_id).into());
        }
        self.document.objects.insert(pages_id, dictionary! {
            "Type" => "Pages", "Kids" => Object::Array(kids), "Count" => self.pages.len() as i64,
        }.into());
        if let Some(Object::Dictionary(catalog)) = self.document.objects.get_mut(&self.catalog_id) {
            catalog.set("Pages", pages_id);
        }
        self.document.trailer.set("Root", self.catalog_id);
        Ok(())
    }

    pub fn save(mut self, path: &str) -> Result<std::fs::File, std::io::Error> {
        if self.pages.is_empty() || !self.operations.is_empty() {
            self.finalize_pages()?;
        }
        self.document.save(path)
    }
    pub fn finish(&mut self) -> Result<Vec<u8>, std::io::Error> {
        if self.pages.is_empty() {
            self.finalize_pages()?;
        }
        let mut buffer = Vec::new();
        self.document
            .save_to(&mut buffer)
            .map_err(|e| std::io::Error::other(e.to_string()))?;
        Ok(buffer)
    }
}
