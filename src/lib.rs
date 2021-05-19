use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Ok,
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
    pub data: Option<MovieList>,
}

#[cfg(test)]
mod tests {
    static TEST_DATA: &str = include_str!("test/test.json");

    use super::*;

    #[test]
    fn deserialize_test_data() {
        let response: Response = serde_json::from_str(TEST_DATA).unwrap();
    }
}
