use yts_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let movie = yts_api::MovieDetails::new(10).execute().await?;
    println!("{:?}", movie);
    Ok(())
}
