use std::process::exit;

use exitfailure::ExitFailure;
use serde::Deserialize;
use reqwest::get;
use inline_colorization::*;

#[derive(Deserialize)]
struct ImgData {
    url: String,
    width: u32,
    height: u32,
}

impl ImgData {
    // Update url to include user input as to how many cats they want
    async fn fetch() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search").await?.json::<(ImgData,)>().await?;
        Ok(Some(img.0))
    }  
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = ImgData::fetch().await?.unwrap();
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() >= 3 {
        println!("Too many arguments. To see usage, use --help");
        exit(-1);
    } else if arg.len() == 1 {
        println!("{style_bold}{color_green}URL: {color_reset}{style_reset}{color_blue}{}{color_reset} \n{style_bold}{color_green}Resolution:{color_reset}{style_reset} {color_blue}{}x{}{color_reset}", img.url, img.width, img.height);
    } else {
        match arg[1].as_str().trim() {
            "--nocolor" | "-n" => println!("URL: {} \nResolution: {}x{}", img.url, img.width, img.height),
            "--help" | "-h" => println!("--help/-h      displays the current help page\n--nocolor/-n    disables colored text in response"),
            _ => {
                println!("Invalid argument. To see usage, use --help");
                exit(-1);
            }
        }
    }
    Ok(())
}


