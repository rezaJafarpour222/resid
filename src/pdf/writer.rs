use lopdf::{Document, Object, content::Operation, dictionary};

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
    pub font: Option<Font>,
    pub font_installed: bool,
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
            font: None,
            font_installed: false,
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
}
