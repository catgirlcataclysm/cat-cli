use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, short)]
    pub test: String,
}