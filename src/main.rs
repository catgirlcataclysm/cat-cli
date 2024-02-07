use reqwest::get;
use reqwest::Error;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit::dpi::LogicalSize;
use serde::Deserialize;

#[derive(Deserialize)]
struct ImgData {
    id: String,
    url: String,
    width: u32,
    height: u32,
}

impl ImgData {
    async fn get_url() -> Result<String, Error> {
        let data = get("https://api.thecatapi.com/v1/images/search").await?.json::<ImgData>().await?;
        let url = data.url;
        return Ok(url)
        // TODO: FIX THIS SHIT
    }
}

fn make_window(transparent: bool, blur: bool) {

    let event_loop = EventLoop::new().expect("Failed to create EventLoop");
    let window_create = WindowBuilder::new();
    window_create
        .with_min_inner_size(LogicalSize::new(200.0, 200.0))
        .with_transparent(transparent)
        .with_blur(blur)
        .with_title("C.A.T. : Cat Aquiring Tool")
        .build(&event_loop)
        .expect("Failed to create window");
}

fn main() {
    //  Creates the window using variables for transparency and blur
    // TODO: Allow those options to be set by users in app
    make_window(true, true);
    // TODO: IMPLEMENT GUI TO FETCH NEW IMAGE
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get a random cat image
}
