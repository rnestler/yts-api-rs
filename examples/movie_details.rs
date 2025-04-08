#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let movie = yts_api::MovieDetails::new(10)
        .with_images(true)
        .with_cast(true)
        .execute()
        .await?;
    println!("{:#?}", movie);
    Ok(())
}
