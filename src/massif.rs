use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};

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
pub fn fetch_examples(query: &str) -> Result<Response, Box<dyn std::error::Error>> {
    // Indeterminate progressbar
    let mut sp = Spinner::new(Spinners::Shark, format!("Fetching {query} from Massif...\n").into());

    // Make a blocking API request
    let text = reqwest::blocking::get(MASSIF_URL.replace("{}", query))?.text()?;
    let json: Response = serde_json::from_str(&text)?;

    sp.stop();

    Ok(json)
}
