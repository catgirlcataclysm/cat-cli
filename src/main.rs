use std::process::exit;
use exitfailure::ExitFailure;
use inline_colorization::*;

mod img;

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = img::ImgData::fetch().await?.expect("Can't reach the cat API.");
    // Clear the terminal before printing image
    print!("\x1B[2J");
    // Print image
    img::ImgData::print_img(&img.url).await.expect("Failed to print to the terminal");
    
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() >= 3 {
        println!("Too many arguments. To see usage, use --help");
        exit(-1);
    } else if arg.len() == 1 {
        println!("{style_bold}{color_green}URL: {color_reset}{style_reset}{color_blue}{}{color_reset} \n{style_bold}{color_green}Resolution:{color_reset}{style_reset} {color_blue}{}x{}{color_reset}", &img.url, &img.width, &img.height);
    } else {
        match arg[1].as_str().trim() {
            "--nocolor" | "-c" => println!("URL: {} \nResolution: {}x{}", &img.url, &img.width, &img.height),
            "--help" | "-h" => println!("--help/-h      displays the help page\n--nocolor/-n    disables colored text in response"),
            "--noimage" | "-i" => println!("{style_bold}{color_green}URL: {color_reset}{style_reset}{color_blue}{}{color_reset} \n{style_bold}{color_green}Resolution:{color_reset}{style_reset} {color_blue}{}x{}{color_reset}", &img.url, &img.width, &img.height), 
            _ => {
                println!("Invalid argument. To see usage, use --help");
                exit(-1);
            }
        }
    }
    Ok(())
}


