mod services;

use std::{ascii, fs};
use clap::Parser;
use image::{open, DynamicImage, ImageError};
use crate::services::img_to_ascii;

#[derive(Parser)]
#[command(name = "img-ascii")]
#[command(about = "Convert images to ASCII art")]
#[derive(Debug)]
struct Cli {
    input: String,

    #[arg(long, default_value="console")]
    output: String,

    #[arg(long, default_value="name")]
    name: String,

    #[arg(long)]
    file: Option<String>,

    #[arg(long, default_value="style")]
    style: String,

    #[arg(long, default_value = "80")]
    width: u32,
}

fn main() {
    let args = Cli::parse();

    let img = match load_image(&args.input) {
        Ok(image) => image,
        Err(e) => {
            eprintln!("Erreur lors du chargement de l'image: {}", e);
            return
        }
    };

    let ascii_lines = img_to_ascii(&img, args.width, &args.style);


    match args.output.as_str() {
        "console" => {
            for ascii_line in ascii_lines {
                println!("{}",ascii_line);
            }
        },
        "file" => {
            fs::write(args.name, ascii_lines.join("\n"))
                .unwrap_or_else(|e| eprintln!("Erreur: {}", e))
        },
        _ => {
            for ascii_line in ascii_lines {
                println!("{}",ascii_line);
            }
        }
    }
}

fn load_image(path: &str) -> Result<DynamicImage, ImageError> {
    open(path)
}
