use yts_api;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let movie_list = yts_api::ListMovies::new()
        .query_term("test")
        .execute()
        .await?;
    for movie in movie_list.movies {
        println!("{:?}", movie);
    }
    Ok(())
}
