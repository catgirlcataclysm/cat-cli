use exitfailure::ExitFailure;
use reqwest::get;
use serde::Deserialize;

#[cfg(target_os = "linux")]
use std::fs::write;

#[cfg(target_os = "linux")]
use std::path::Path;

#[cfg(target_os = "linux")]
use viuer::{print, Config};

#[derive(Deserialize)]
pub struct ImgData {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

impl ImgData {
    pub async fn fetch() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search")
            .await?
            .json::<(ImgData,)>()
            .await?;
        Ok(Some(img.0))
    }
    #[cfg(target_os = "linux")]
    pub async fn write_img(
        url: &String,
        width: u32,
        height: u32,
        noimg: bool,
        output: Option<String>,
    ) -> Result<(), ExitFailure> {
        let img_bytes = get(url).await?.bytes().await?;
        let img = image::load_from_memory(&img_bytes)?;
        let conf = Config {
            width: Some(width),
            height: Some(height),
            ..Default::default()
        };

        if !noimg {
            //clear the screen
            print!("\x1B[2J");
            print(&img, &conf).expect("Image printing failed.");
        }

        if output.is_some() {
            let output = output.unwrap();
            let path = Path::new(output.as_str());
            write(path, img_bytes).expect("Failed to write to file");
        }
        Ok(())
    }
}
