use std::fs;

use clap::Parser;
use resid::{
    cli::Args,
    composition::engine::CompositionEngine,
    document::{layout_engine::LayoutEngine, page::get_page},
    error::AppError,
    font::loader::Font,
    pdf::writer::PdfWriter,
};

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let html = if args.from.ends_with(".html") {
        fs::read_to_string(&args.from)?
    } else {
        args.from.clone()
    };

    let page = get_page(&args.page).expect("page was already validated by clap");

    let composition = CompositionEngine::new(page);
    let document = composition.compose(&html)?;

    let font = Font::get_font(&args.font)?;

    println!("Font: {}", font.family);
    println!("Page: {}", args.page);

    let layout_engine = LayoutEngine::new(&font);
    let layout = layout_engine.create_layout(&document)?;

    let mut writer = PdfWriter::new(document.page.width, document.page.height);

    writer.set_font(font);

    let shaped_texts = layout
        .pages
        .iter()
        .flat_map(|page| page.blocks.iter())
        .flat_map(|block| block.content.lines.iter())
        .map(|line| line.glyphs.clone())
        .collect::<Vec<_>>();

    if !shaped_texts.is_empty() {
        writer.install_font(&shaped_texts)?;
    }

    for (page_index, page) in layout.pages.iter().enumerate() {
        if page_index > 0 {
            writer.new_page();
        }

        for block in &page.blocks {
            writer.draw_layout_block(block)?;
        }
    }

    writer.save(&args.create)?;

    println!("PDF created: {}", args.create);

    Ok(())
}
