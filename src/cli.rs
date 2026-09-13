use clap::{Parser, builder::PossibleValuesParser};

use crate::{document::page::PAGES, font::loader::FONTS};

#[derive(Parser, Debug)]
#[command(name = "resid", version, about = "HTML/CSS to PDF renderer")]
pub struct Args {
    #[arg(long, short = 'f', value_name = "FILE")]
    pub from: String,

    #[arg(long, short = 'c', value_name = "FILE")]
    pub create: String,

    #[arg(
        long,
        default_value = "Vazirmatn",
        value_parser = font_parser()
    )]
    pub font: String,

    #[arg(
        long,
        default_value = "a4",
        value_parser = page_parser()
    )]
    pub page: String,
}

fn font_parser() -> PossibleValuesParser {
    PossibleValuesParser::new(FONTS.iter().map(|font| font.family))
}

fn page_parser() -> PossibleValuesParser {
    PossibleValuesParser::new(PAGES.iter().map(|page| page.name))
}
