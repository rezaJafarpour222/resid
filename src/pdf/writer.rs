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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        composition::engine::CompositionEngine,
        document::{layout_engine::LayoutEngine, page::Page},
        font::{loader::Font, shaper::Shaper},
        units::{Direction, Millimeter, Pt},
    };

    fn test_font() -> Font {
        super::Font::get_font("B-Nazanin").unwrap()
    }

    #[test]
    fn creates_a4_pdf_with_rectangle() {
        let width: Pt = Millimeter::new(210.0).into();
        let height: Pt = Millimeter::new(297.0).into();
        let mut writer = PdfWriter::new(width, height);

        writer.draw_rectangle(
            Pt::new(100.0),
            Pt::new(100.0),
            Pt::new(200.0),
            Pt::new(100.0),
        );

        let bytes = writer.finish().expect("failed to finish PDF");
        assert!(!bytes.is_empty());
    }

    #[test]
    fn creates_pdf_with_multiple_persian_lines() {
        let width: Pt = Millimeter::new(210.0).into();
        let height: Pt = Millimeter::new(297.0).into();
        let font = test_font();

        let texts = ["فاکتور فروش", "شماره فاکتور: 1001", "سلام دنیا", "چطوری غلام"];
        let shaped_texts = texts
            .iter()
            .map(|text| Shaper::shaped_text(&font, text, Direction::RTL, Pt::new(24.0)))
            .collect::<Result<Vec<_>, _>>()
            .expect("failed to shape text");

        let mut writer = PdfWriter::new(width, height);
        writer.set_font(font);
        writer
            .install_font(&shaped_texts)
            .expect("failed to install font");

        for (index, shaped) in shaped_texts.iter().enumerate() {
            writer
                .draw_shaped_text(
                    shaped,
                    Pt::new(100.0),
                    Pt::new(100.0 + index as f32 * 40.0),
                    Pt::new(24.0),
                    crate::css::types::Color::BLACK,
                    crate::css::types::FontWeight::Normal,
                )
                .expect("failed to draw text");
        }

        let bytes = writer.finish().expect("failed to finish PDF");
        assert!(!bytes.is_empty());
    }

    #[test]
    fn creates_pdf_with_composed_wrapped_persian_paragraph() {
        let font = test_font();
        let html = r#"
            <html>
                <head>
                    <style>
                        body { direction: rtl; }
                        p { font-size: 12pt; line-height: 1.5; text-align: right; }
                    </style>
                </head>
                <body>
                    <p> این یک متن فارسی باین یک متن فارسی ب
                    این یک متن فارسی باین یک متن فارسی باین یک متن ف
                    ارسی باین یک متن فارسی بسیار بسیار بسیار بسیار بسیار بسیار طولانی است که باید در چند خط قرار بگیرد</p>
                </body>
            </html>
        "#;

        let composition = CompositionEngine::new(Page::a4_portrait());
        let document = composition.compose(html).expect("composition failed");
        let layout = LayoutEngine::new(&font)
            .create_layout(&document)
            .expect("layout failed");

        let paragraph = layout.pages[0]
            .blocks
            .iter()
            .find(|block| !block.content.lines.is_empty())
            .expect("paragraph block missing");

        assert!(paragraph.content.lines.len() > 1);

        let shaped_texts = paragraph
            .content
            .lines
            .iter()
            .map(|line| line.glyphs.clone())
            .collect::<Vec<_>>();

        let mut writer = PdfWriter::new(document.page.width, document.page.height);
        writer.set_font(font);
        writer
            .install_font(&shaped_texts)
            .expect("failed to install font");
        writer
            .draw_layout_block(paragraph)
            .expect("failed to draw paragraph");

        let bytes = writer.finish().expect("failed to finish PDF");
        assert!(!bytes.is_empty());
    }

    #[test]
    fn lays_out_nested_blocks_with_spacing_and_styles() {
        let font = test_font();
        let html = r#"
            <html lang="fa" dir="rtl">
                <head>
                    <style>
                        body { margin: 10pt; direction: rtl; }
                        .outer { margin: 10pt; padding: 10pt; background: #eeeeee; border: 1pt solid #000000; }
                        .title { margin: 5pt; padding: 5pt; text-align: center; font-size: 20pt; }
                        .text { margin: 5pt; padding: 5pt; text-align: right; font-size: 12pt; }
                    </style>
                </head>
                <body>
                    <div class="outer">
                        <div class="title">فاکتور فروش</div>
                        <div class="text">این یک متن فارسی طولانی است که باید در چند خط شکسته شود و درون بلوک خود قرار بگیرد.</div>
                        <div class="text">شماره فاکتور: ۱۴۰۵-۰۰۱۲۵</div>
                    </div>
                </body>
            </html>
        "#;

        let document = CompositionEngine::new(Page::a4_portrait())
            .compose(html)
            .expect("composition failed");
        let layout = LayoutEngine::new(&font)
            .create_layout(&document)
            .expect("layout failed");

        assert!(layout.pages[0].blocks.len() >= 4);
        assert!(
            layout.pages[0]
                .blocks
                .iter()
                .any(|block| block.background.is_some())
        );
        assert!(
            layout.pages[0]
                .blocks
                .iter()
                .any(|block| !block.content.lines.is_empty())
        );
    }
}
