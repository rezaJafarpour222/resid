use crate::{
    composition::engine::CompositionEngine,
    document::{layout_engine::LayoutEngine, page::Page},
    font::{loader::Font, shaper::Shaper},
    pdf::writer::PdfWriter,
    units::{Direction, Millimeter, Pt},
};

// SECTION: Common
const FONTS: &[&str] = &[
    "Estedad",
    "Gandom",
    "Parastoo",
    "Samim",
    "Shabnam",
    "Vazirmatn",
];
fn load_font(name: &str) -> Font {
    Font::get_font(name).unwrap()
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
    for f in FONTS {
        let font = load_font(f);
        let width: Pt = Millimeter::new(210.0).into();
        let height: Pt = Millimeter::new(297.0).into();
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
}

#[test]
fn creates_pdf_with_composed_wrapped_persian_paragraph() {
    for f in FONTS {
        let font = load_font(f);
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
}

#[test]
fn lays_out_nested_blocks_with_spacing_and_styles() {
    for f in FONTS {
        let font = load_font(f);
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
                        <div class="title"> فاکتور فروش</div>
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
