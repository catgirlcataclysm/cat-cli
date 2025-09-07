use exitfailure::ExitFailure;

mod args;
mod img;

// TODO
// ALLOW MULTIPLE DOWNLOADS(?)

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    // Queries the API @ https://api.thecatapi.com/v1/images/search to get ImgData for a random cat image
    let img = img::ImgData::fetch()
        .await?
        .expect("Can't reach the cat API.");

    args::handle_args(&img).await;

    Ok(())
}