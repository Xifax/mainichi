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

pub fn fetch_examples(query: &str) -> Result<Response, Box<dyn std::error::Error>> {
    let text = reqwest::blocking::get(MASSIF_URL.replace("{}", query))?.text()?;
    let json: Response = serde_json::from_str(&text)?;

    Ok(json)

    // for example in json.results.iter().take(3) {
    //     println!("{:#?}", example.text);
    // }

    // Ok(())
}
