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

    pub fn save(mut self, path: &str) -> Result<std::fs::File, std::io::Error> {
        let content = Content {
            operations: self.operations,
        };

        let content_data = content
            .encode()
            .map_err(|error| std::io::Error::other(error.to_string()))?;

        let content_id = self.document.new_object_id();

        self.document.objects.insert(
            content_id,
            Object::Stream(lopdf::Stream::new(dictionary! {}, content_data)),
        );

        if let Some(Object::Dictionary(page)) = self.document.objects.get_mut(&self.page_id) {
            page.set("Contents", content_id);
        }

        self.document.save(path)
    }
    pub fn finish(&mut self) -> Result<Vec<u8>, std::io::Error> {
        let mut buffer = Vec::new();

        self.document
            .save_to(&mut buffer)
            .map_err(|error| std::io::Error::other(error.to_string()))?;

        Ok(buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        composition::engine::CompositionEngine,
        document::{layout_engine::LayoutEngine, types::Page},
        font::{loader::Font, shaper::Shaper},
        units::{Direction, Millimeter, Pt},
    };

    fn test_font() -> Font {
        Font::load("B NAZANIN", include_bytes!("../../resources/B-NAZANIN.TTF"))
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

        let composition = CompositionEngine::new(Page::a4());
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

        let document = CompositionEngine::new(Page::a4())
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
