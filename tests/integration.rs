use std::{fs, path::PathBuf};

use resid::{
    composition::engine::CompositionEngine,
    document::{layout_engine::LayoutEngine, page::Page},
    font::loader::Font,
    pdf::writer::PdfWriter,
};

fn example_html_file(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join(name);

    fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()))
}

fn create_pdf(html: &str, file_name: &str, font: &Font, page: Page) {
    let composition = CompositionEngine::new(page);
    let document = composition
        .compose(html)
        .expect("failed to compose document");

    let layout_engine = LayoutEngine::new(font);
    let layout = layout_engine
        .create_layout(&document)
        .expect("failed to create layout");

    let mut writer = PdfWriter::new(document.page.width, document.page.height);

    writer.set_font(font.clone());

    let shaped_texts = layout
        .pages
        .iter()
        .flat_map(|page| page.blocks.iter())
        .flat_map(|block| block.content.lines.iter())
        .map(|line| line.glyphs.clone())
        .collect::<Vec<_>>();

    if !shaped_texts.is_empty() {
        writer
            .install_font(&shaped_texts)
            .expect("failed to install font");
    }

    for (page_index, page) in layout.pages.iter().enumerate() {
        if page_index > 0 {
            writer.new_page();
        }

        for block in &page.blocks {
            writer
                .draw_layout_block(block)
                .expect("failed to draw layout block");
        }
    }

    writer.save(file_name).expect("failed to save PDF");
}

#[test]
#[ignore = "integration test"]
fn create_pdf_from_html() {
    let html = example_html_file("invoice2.html");
    let font = Font::get_font("Vazirmatn").expect("Vazirmatn font not found");

    let output = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("test-output.pdf");

    create_pdf(
        &html,
        output.to_str().expect("invalid output path"),
        &font,
        Page::a3(),
    );
}
