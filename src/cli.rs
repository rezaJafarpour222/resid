use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "resid")]
pub struct Args {
    #[arg(long)]
    pub create: String,

    #[arg(long)]
    pub from: String,
}
