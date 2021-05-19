use yts_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let movie_list = yts_api::list_movies("test").await?;
    for movie in movie_list.movies {
        println!("{:?}", movie);
    }
    Ok(())
}
