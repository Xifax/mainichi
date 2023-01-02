use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub results: Vec<Example>,
}

#[derive(Deserialize, Debug)]
pub struct Example {
    pub text: String,
}

const MASSIF_URL: &str = "https://massif.la/ja/search?q={}&fmt=json";

/// Download 100 examples from Massif.la if available
/// Order is not randomized but graded (according to the resource)
pub fn fetch_examples(query: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let text = reqwest::blocking::get(MASSIF_URL.replace("{}", query))?.text()?;
    let json: Response = serde_json::from_str(&text)?;

    Ok(json)
}
