#[cfg(any(target_os = "linux"))]
use viuer::{print, Config};

use serde::Deserialize;
use exitfailure::ExitFailure;
use reqwest::get;


#[derive(Deserialize)]
pub struct ImgData {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl ImgData {
    pub async fn fetch() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search").await?.json::<(ImgData,)>().await?;
        Ok(Some(img.0))
    }
    #[cfg(any(target_os = "linux"))]
    pub async fn print_img(url: &String) -> Result<(), ExitFailure> {
        let img_bytes = get(url).await?.bytes().await?;
        let img = image::load_from_memory(&img_bytes)?;
        let conf = Config {
            width: Some(128),
            height: Some(72),
            ..Default::default()
        };
        print(&img, &conf).expect("Image printing failed.");
        Ok(())
    }
}