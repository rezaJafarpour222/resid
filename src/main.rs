use std::fs;

use clap::Parser;

use crate::{
    cli::Args,
    composition::engine::CompositionEngine,
    document::{layout_engine::LayoutEngine, types::Page},
    error::AppError,
    font::loader::Font,
    pdf::writer::PdfWriter,
};

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

    let font = Font::load("B-Nazanin", include_bytes!("../resources/B-NAZANIN.TTF"));

    let composition = CompositionEngine::new(Page::a4());
    let document = composition.compose(&html)?;

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

    for page in &layout.pages {
        for block in &page.blocks {
            writer.draw_layout_block(block)?;
        }
    }
    writer.finish()?;
    writer.save(&args.create)?;

    println!("PDF created: {}", args.create);

    Ok(())
}
