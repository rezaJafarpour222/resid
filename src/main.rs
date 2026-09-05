use crate::{
    cli::Args,
    composition::engine::CompositionEngine,
    document::{layout_engine::LayoutEngine, page::Page},
    error::AppError,
    font::loader::Font,
    pdf::writer::PdfWriter,
};
use clap::Parser;
use std::fs;
pub mod cli;
pub mod composition;
pub mod css;
pub mod document;
pub mod error;
pub mod font;
pub mod html;
pub mod pdf;
pub mod units;

fn main() -> Result<(), AppError> {
    let args = Args::parse();

    let html = if args.from.ends_with(".html") {
        fs::read_to_string(&args.from)?
    } else {
        args.from.clone()
    };

    let page = match args.page.as_str() {
        "a3" => Page::a3(),
        "a4-landscape" => Page::a4_landscape(),
        "a5" => Page::a5(),
        "a6" => Page::a6(),
        _ => Page::a4_portrait(),
    };

    let composition = CompositionEngine::new(page);
    let document = composition.compose(&html)?;
    let font = Font::get_font("Vazirmatn")?;
    println!("{:}", font.family);
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
