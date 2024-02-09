use serde::Deserialize;
use exitfailure::ExitFailure;
use reqwest::get;


#[derive(Deserialize)]
pub struct ImgData {
    pub url: String,
    pub width: usize,
    pub height: usize,
}

impl ImgData {
    // Update url to include user input as to how many cats they want
    pub async fn fetch() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search").await?.json::<(ImgData,)>().await?;
        Ok(Some(img.0))
    }
    pub async fn sixel(url: &String) -> Result<(), ExitFailure> {
        let img_bytes = get(url).await?.bytes().await?;
        Ok(())
    }
}