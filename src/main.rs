use std::fs;

use clap::Parser;

use crate::{document::layout_engine::LayoutEngine, error::AppError, font::loader::Font};

pub mod cli;
pub mod document;
pub mod error;
pub mod font;
pub mod html;
pub mod pdf;
pub mod units;

fn main() -> Result<(), AppError> {
    let args = cli::Args::parse();
    let mut html: String;
    if args.from.ends_with(".html") {
        html = match fs::read_to_string(&args.from) {
            Ok(html) => html,
            Err(error) => {
                eprintln!("failed to read {}: {error}", args.from);
                std::process::exit(1);
            }
        };
    } else {
        html = args.from.clone();
    }
    let document = html::parser::HtmlBuilder::parse(html.as_mut_str())?;
    let font = Font::load("B-Nazanin", "B-NAZANIN.TTF")?;
    let layout_engine = LayoutEngine::new(&font);
    let layout = layout_engine.create_layout(&document)?;
    let mut writer = pdf::writer::PdfWriter::new(document.page.width, document.page.height);
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
            for line in &block.content.lines {
                writer.draw_layout_line(line)?;
            }
        }
    }
    let pdf = writer.finish()?;

    fs::write(&args.create, pdf)?;

    println!("PDF created: {}", args.create);

    Ok(())
}
