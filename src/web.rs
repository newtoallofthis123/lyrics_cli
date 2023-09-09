use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Response {
    pub url: String,
    pub lyrics: String,
}

pub async fn get_lyrics(query: &str) -> (String, String) {
    let url = format!("https://lyrics.astrid.sh/api/search?q={}", query);
    let resp = reqwest::get(&url).await.unwrap();
    //use serde_json to parse the response
    let json: Response = serde_json::from_str(&resp.text().await.unwrap()).unwrap();
    (json.url, json.lyrics)
}