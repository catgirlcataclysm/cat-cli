use reqwest::get;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use exitfailure::ExitFailure;
use serde::Deserialize;

#[derive(Deserialize)]
struct ImgData {
    id: String,
    url: String,
    width: u32,
    height: u32,
}

impl ImgData {
  async fn get() -> Result<Option<ImgData>, ExitFailure> {
        let img = get("https://api.thecatapi.com/v1/images/search").await?.json::<(ImgData,)>().await?;
        //  remove will panic if less than 2 items in imgdata
        Ok(Some(img.0))
    }  
}

fn make_window() {
    let event_loop = EventLoop::new().expect("Failed to create EventLoop");
    let window_create = WindowBuilder::new();
    window_create
        .with_min_inner_size(LogicalSize::new(200.0, 200.0))
        .with_title("C.A.T. : Cat Aquiring Tool")
        .build(&event_loop)
        .expect("Failed to create window");
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    //  Creates the window using variables for transparency and blur
    // TODO: Allow those options to be set by users in app
    make_window();
    // TODO: IMPLEMENT GUI TO FETCH NEW IMAGE
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = ImgData::get().await?.unwrap();
    println!("{} {} {} {}", img.url, img.width, img.height, img.id);
    Ok(())


}
