use clap::Parser;
use colored::Colorize;

use crate::img::ImgData;

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Args {
    #[arg(
        long,
        short = 'c',
        help = "Print URL and Resolution text without color"
    )]
    pub nocolor: bool,

    #[arg(long, short = 'u', help = "Don't print the URL of the image")]
    pub nourl: bool,

    #[arg(long, short = 'r', help = "Don't print the resolution of the image")]
    pub nores: bool,

    #[arg(
        long,
        short = 'W',
        help = "Sets the width of the image displayed in the terminal",
        default_value_t = 32
    )]
    pub width: u32,

    #[arg(
        long,
        short = 'H',
        help = "Sets the height of the image displayed in the terminal",
        default_value_t = 18
    )]
    pub height: u32,

    #[arg(long, short = 'i', help = "Don't print the image in the terminal")]
    pub noimg: bool,

    #[arg(
        long,
        short = 'o',
        help = "Sets the output file for the image to be saved to"
    )]
    pub output: Option<String>,
}

pub async fn handle_args(img: &ImgData) {
    let args = Args::parse();

    #[cfg(target_os = "linux")]
    crate::img::ImgData::write_img(&img.url, args.width, args.height, args.noimg, args.output)
        .await
        .expect("Failed printing to the terminal.");

    if args.nourl & !args.nores {
        if args.nocolor {
            println!("{} {}x{}", "Resolution:".bold(), img.width, img.height);
        } else {
            println!(
                "{} {}{}{}",
                "Resolution:".green().bold(),
                img.width.to_string().blue(),
                "x".blue(),
                img.height.to_string().blue()
            );
        }
    } else if args.nores & !args.nourl {
        if args.nocolor {
            println!("{} {}", "URL:".bold(), img.url);
        } else {
            println!("{} {}", "URL:".green().bold(), img.url.blue());
        }
    } else if !(args.nores & args.nourl) {
        if args.nocolor {
            println!("{} {}", "URL:".bold(), img.url);
            println!("{} {}x{}", "Resolution:".bold(), img.width, img.height);
        } else {
            println!("{} {}", "URL:".green().bold(), img.url.blue());
            println!(
                "{} {}{}{}",
                "Resolution:".green().bold(),
                img.width.to_string().blue(),
                "x".blue(),
                img.height.to_string().blue()
            );
        }
    }
}
