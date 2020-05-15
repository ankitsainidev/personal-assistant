/* Uses simple Quotable API to recieve random quote.
More about API at https://github.com/lukePeavey/quotable */
use reqwest;
use serde;
use serde_json;
use std::fmt;
const API_URL: &'static str = "https://api.quotable.io/random";

#[derive(Debug, serde::Deserialize)]
pub struct Quote {
    _id: String,
    tags: Vec<String>,
    content: String,
    author: String,
    length: i32,
}
impl fmt::Display for Quote {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{content}       - {author}",
            content = self.content,
            author = self.author
        )
    }
}
pub async fn quote() -> Result<Quote, reqwest::Error> {
    let data = reqwest::get(API_URL).await?.text().await?;
    Ok(serde_json::from_str(&data).unwrap())
}

#[tokio::test]
async fn api_connection() {
    quote().await.expect("Error connecting to api");
}
