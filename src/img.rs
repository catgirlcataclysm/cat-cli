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
    pub async fn fetch() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search").await?.json::<(ImgData,)>().await?;
        Ok(Some(img.0))
    }
    async fn gen_img(url: String) -> Result<(), ExitFailure> {
        let res = get(url).await.expect("Failed to get image").bytes().await?;
        let byte_vec = Vec::from(res);
        let img = image::DynamicImage::ImageRgba8((/*RGBAU8VEC*/, byte_vec));
        Ok(())
    }
}