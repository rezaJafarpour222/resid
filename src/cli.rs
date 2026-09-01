use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub create: String,

    #[arg(long)]
    pub from: String,
}
