use hyper::{body::HttpBody, Client};
use hyper_tls::HttpsConnector;
use parse_display::Display;
use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
    Other(String),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Torrent {
    pub url: String,
    pub hash: String,
    pub quality: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub seeds: u32,
    pub peers: u32,
    pub size: String,
    pub size_bytes: u64,
    pub date_uploaded: String,
    pub date_uploaded_unix: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Movie {
    pub id: u32,
    pub url: String,
    pub imdb_code: String,
    pub title: String,
    pub title_english: String,
    pub title_long: String,
    pub slug: String,
    pub year: u32,
    pub rating: f32,
    pub runtime: u32,
    pub genres: Vec<String>,
    pub summary: String,
    pub description_full: String,
    pub synopsis: String,
    pub yt_trailer_code: String,
    pub language: String,
    pub mpa_rating: String,
    pub background_image: String,
    pub background_image_original: String,
    pub small_cover_image: String,
    pub medium_cover_image: String,
    pub large_cover_image: String,
    pub state: Status,
    pub torrents: Vec<Torrent>,
    pub date_uploaded: String,
    pub date_uploaded_unix: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MovieList {
    pub movie_count: u32,
    pub limit: u32,
    pub page_number: u32,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub movies: Vec<Movie>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Data {
    MovieList(MovieList),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Response {
    pub status: Status,
    pub status_message: String,
    pub data: Option<Data>,
}

#[derive(Display, Copy, Clone, Debug)]
pub enum Quality {
    #[display("720p")]
    Q720p,
    #[display("1080p")]
    Q1080p,
    #[display("2160p")]
    Q2160p,
    #[display("3D")]
    Q3D,
}

#[derive(Display, Copy, Clone, Debug)]
#[display(style = "snake_case")]
pub enum Sort {
    Title,
    Year,
    Rating,
    Peers,
    Seeds,
    DownloadCount,
    LikeCount,
    DateAdded,
}

#[derive(Display, Copy, Clone, Debug)]
#[display(style = "snake_case")]
pub enum Order {
    Desc,
    Asc,
}

pub trait ApiEndpoint {
    fn get_url(&self) -> String;
}

fn add_query(url: &mut String, name: &str, value: Option<impl fmt::Display>) {
    if let Some(value) = value {
        url.push_str(&format!("{}={}&", name, value));
    }
}

async fn execute(
    api_endpoint: &dyn ApiEndpoint,
) -> Result<Data, Box<dyn std::error::Error + Send + Sync>> {
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let url = api_endpoint.get_url();

    let mut res = client.get(url.parse()?).await?;
    let mut bytes = Vec::new();
    while let Some(next) = res.data().await {
        let chunk = next?;
        bytes.extend(chunk);
    }
    let body = String::from_utf8(bytes)?;
    let response: Response = serde_json::from_str(&body)?;
    if let Status::Other(status) = response.status {
        return Err(format!("{}: {}", status, response.status_message).into());
    }
    let data = response.data.ok_or("Data missing")?;
    Ok(data)
}

#[derive(Clone, Debug, Default)]
pub struct ListMovies<'a> {
    /// The limit of results per page that has been set
    limit: Option<u8>,
    /// Used to see the next page of movies, eg limit=15 and page=2 will show you movies 15-30
    page: Option<u32>,
    /// Used to filter by a given quality
    quality: Option<Quality>,
    /// Used to filter movie by a given minimum IMDb rating
    minimum_rating: Option<u8>,
    /// Used for movie search, matching on: Movie Title/IMDb Code, Actor Name/IMDb Code, Director
    /// Name/IMDb Code
    query_term: Option<&'a str>,
    /// Used to filter by a given genre (See http://www.imdb.com/genre/ for full list)
    genre: Option<&'a str>,
    /// Sorts the results by choosen value
    sort_by: Option<Sort>,
    /// Orders the results by either Ascending or Descending order
    order_by: Option<Order>,
    /// Returns the list with the Rotten Tomatoes rating included
    wirth_rt_ratings: Option<bool>,
}

impl<'a> ListMovies<'a> {
    pub fn new() -> ListMovies<'a> {
        ListMovies::default()
    }

    pub fn limit(&mut self, limit: u8) -> &mut Self {
        assert!(limit > 1 && limit <= 50, "limit out of range");
        self.limit = Some(limit);
        self
    }

    pub fn page(&mut self, page: u32) -> &mut Self {
        assert!(page > 1, "page out of range");
        self.page = Some(page);
        self
    }

    pub fn quality(&mut self, quality: Quality) -> &mut Self {
        self.quality = Some(quality);
        self
    }

    pub fn query_term(&mut self, query_term: &'a str) -> &mut Self {
        self.query_term = Some(query_term);
        self
    }

    pub fn genre(&mut self, genre: &'a str) -> &mut Self {
        self.genre = Some(genre);
        self
    }

    pub fn sort_by(&mut self, sort_by: Sort) -> &mut Self {
        self.sort_by = Some(sort_by);
        self
    }

    pub fn order_by(&mut self, order_by: Order) -> &mut Self {
        self.order_by = Some(order_by);
        self
    }

    pub fn wirth_rt_ratings(&mut self, wirth_rt_ratings: bool) -> &mut Self {
        self.wirth_rt_ratings = Some(wirth_rt_ratings);
        self
    }

    pub async fn execute(&self) -> Result<MovieList, Box<dyn std::error::Error + Send + Sync>> {
        let data = execute(self).await?;
        match data {
            Data::MovieList(movie_list) => Ok(movie_list),
        }
    }
}

impl<'a> ApiEndpoint for ListMovies<'a> {
    fn get_url(&self) -> String {
        let mut url = "https://yts.mx/api/v2/list_movies.json?".to_owned();

        add_query(&mut url, "limit", self.limit);
        add_query(&mut url, "page", self.page);
        add_query(&mut url, "quality", self.quality);
        add_query(&mut url, "minimum_rating", self.minimum_rating);
        add_query(&mut url, "query_term", self.query_term);
        add_query(&mut url, "genre", self.genre);
        add_query(&mut url, "sort_by", self.sort_by);
        add_query(&mut url, "order_by", self.order_by);
        add_query(&mut url, "wirth_rt_ratings", self.wirth_rt_ratings);
        url
    }
}


        }
    }
}

#[deprecated(
    since = "0.1.2",
    note = "Use ListMovies::new().query_term(...).execute() instead"
)]
pub async fn list_movies(
    query_term: &str,
) -> Result<MovieList, Box<dyn std::error::Error + Send + Sync>> {
    ListMovies::new().query_term(query_term).execute().await
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = include_str!("test/test.json");

    use super::*;

    #[test]
    fn list_movies_url_build_empty() {
        let url = ListMovies::new().get_url();
        assert_eq!(url, "https://yts.mx/api/v2/list_movies.json?");
    }

    #[test]
    fn list_movies_url_query_term() {
        let url = ListMovies::new().query_term("test").get_url();
        assert_eq!(
            url,
            "https://yts.mx/api/v2/list_movies.json?query_term=test&"
        );
    }

    #[test]
    fn deserialize_test_data() {
        let response: Response = serde_json::from_str(TEST_DATA).unwrap();
        assert_eq!(response.status, Status::Ok);
        assert_eq!(response.status_message, "Query was successful");
        let data = response.data.unwrap();
        let movie_list = match data {
            Data::MovieList(movie_list) => movie_list,
        };
        assert_eq!(movie_list.movie_count, 10);
        assert_eq!(movie_list.limit, 20);
        assert_eq!(movie_list.page_number, 1);
        assert_eq!(movie_list.movies.len(), 10);
    }

    #[test]
    fn deserialize_empty_test_data() {
        static TEST_DATA: &str = include_str!("test/test_empty.json");
        let response: Response = serde_json::from_str(TEST_DATA).unwrap();
        assert_eq!(response.status, Status::Ok);
        assert_eq!(response.status_message, "Query was successful");
        let data = response.data.unwrap();
        let movie_list = match data {
            Data::MovieList(movie_list) => movie_list,
        };
        assert_eq!(movie_list.movie_count, 0);
        assert_eq!(movie_list.limit, 20);
        assert_eq!(movie_list.page_number, 1);
        assert_eq!(movie_list.movies.len(), 0);
    }
}
