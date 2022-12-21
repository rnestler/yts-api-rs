use std::env;

use yts_api::{ListMovies, MovieList};

async fn search(term: &str) -> Result<MovieList, Box<dyn std::error::Error + Send + Sync>> {
    ListMovies::new().query_term(term).execute().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let search_term = env::args().skip(1).next().unwrap_or("test".to_string());
    let movie_list = search(&search_term).await?;
    for movie in movie_list.movies {
        println!("{:?}", movie);
    }
    Ok(())
}
