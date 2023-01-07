use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Response {
    pub results: Vec<Example>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Example {
    pub text: String,
}

const MASSIF_URL: &str = "https://massif.la/ja/search?q={}&fmt=json";

/// Download 100 examples from Massif.la if available
/// Order is not randomized but graded (according to the resource)
pub async fn fetch_examples(query: &str) -> Result<Response, Box<dyn std::error::Error>> {
    // Make an async API request
    let response = reqwest::get(MASSIF_URL.replace("{}", query)).await?;
    let json = response.json::<Response>().await?;

    Ok(json)
}
