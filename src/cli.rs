use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "resid", version, about = "HTML/CSS to PDF renderer")]
pub struct Args {
    #[arg(long)]
    pub create: String,

    #[arg(long)]
    pub from: String,

    #[arg(long, default_value = "a4", value_parser = parse_page)]
    pub page: String,
}

fn parse_page(value: &str) -> Result<String, String> {
    match value.to_ascii_lowercase().as_str() {
        "a3" | "a4" | "a4-landscape" | "a5" | "a6" => Ok(value.to_ascii_lowercase()),
        _ => Err("page must be one of: a3, a4, a4-landscape, a5, a6".to_string()),
    }
}
