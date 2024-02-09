use std::process::exit;
use exitfailure::ExitFailure;
use inline_colorization::*;

mod img;

#[cfg(any(target_os = "linux"))]
async fn print_img(url: &String) {
    img::ImgData::print_img(url).await.expect("Failed to print to the terminal");
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = img::ImgData::fetch().await?.expect("Can't reach the cat API."); 
    
    let arg: Vec<String> = std::env::args().collect();
    if arg.len() >= 3 {
        println!("Too many arguments. To see usage, use --help");
        exit(-1);
    } else if arg.len() == 1 {
        #[cfg(any(target_os = "linux"))]
        // Clear the terminal before printing image
        print!("\x1B[2J");
        // Print image
        #[cfg(any(target_os = "linux"))]
        print_img(&img.url).await;
        
        println!("{style_bold}{color_green}URL: {color_reset}{style_reset}{color_blue}{}{color_reset} \n{style_bold}{color_green}Resolution:{color_reset}{style_reset} {color_blue}{}x{}{color_reset}", &img.url, &img.width, &img.height);
    } else {
        match arg[1].as_str().trim() {
            "--nocolor" | "-c" => {
                #[cfg(any(target_os = "linux"))]
                // Clear the terminal before printing image
                print!("\x1B[2J");
                // Print image
                #[cfg(any(target_os = "linux"))]
                print_img(&img.url).await;
                
                println!("URL: {} \nResolution: {}x{}", &img.url, &img.width, &img.height);
            }
            "--help" | "-h" => println!("--help/-h      displays the help page\n--nocolor/-c    disables colored text in response\n--noimage/-i  disables the showing of the image in supported terminals"),
            
            "--noimage" | "-i" => println!("{style_bold}{color_green}URL: {color_reset}{style_reset}{color_blue}{}{color_reset} \n{style_bold}{color_green}Resolution:{color_reset}{style_reset} {color_blue}{}x{}{color_reset}", &img.url, &img.width, &img.height), 
            _ => {
                println!("Invalid argument. To see usage, use --help");
                exit(-1);
            }
        }
    }
    Ok(())
}


