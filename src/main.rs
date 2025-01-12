use exitfailure::ExitFailure;

mod img;
mod args;

// TODO

// CLAP CLAP CLAP CLAP CLAP CLAP CLAP CLAP

// DOWNLOAD TO FILE FEATURE
// ALLOW MULTIPLE DOWNLOADS
// HANDLE RATE LIMITING(?)
// ALLOW USER DEFINED SIZE
// HANDLE BEING RUN AS TERMINAL GREETING MESSAGE(????)

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = img::ImgData::fetch().await?.expect("Can't reach the cat API."); 
    
    args::handle_args(&img).await;
    
    
    Ok(())
}


