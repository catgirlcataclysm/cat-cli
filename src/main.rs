use exitfailure::ExitFailure;
use serde::Deserialize;
use reqwest::get;

#[derive(Deserialize)]
struct ImgData {
    url: String,
    width: u32,
    height: u32,
}

impl ImgData {
    async fn fetch() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search").await?.json::<(ImgData,)>().await?;
        Ok(Some(img.0))
    }  
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = ImgData::fetch().await?.unwrap();
    println!("URL: {} \nResolution: {}x{}", img.url, img.width, img.height);
    Ok(())
}
