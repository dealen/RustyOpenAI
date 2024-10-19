use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::json;

#[derive(Default)]
pub struct Moderation {}

impl Moderation {
    #[must_use]
    pub fn new() -> Moderation {
        Moderation {}
    }

    /// # Panics
    /// Panics if the no api key is provided or there is no connection
    pub async fn ask_moderation(&self, key: String, input: String) -> bool {
        let client = reqwest::Client::new();
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&key.to_string()).expect("Could not create header"),
        );

        let data = json!({
            "input": input,
        });

        let res = client
            .post("https://api.openai.com/v1/moderations")
            .headers(headers)
            .json(&data)
            .send()
            .await
            .expect("Could not get moderation reposnse");

        let body = res.text().await.unwrap();

        println!("Moderation response: {body:?}");

        let flagged = body.split("flagged\": ").collect::<Vec<&str>>()[1]
            .split(',')
            .collect::<Vec<&str>>()[0]
            .to_string();

        flagged == "true"
    }
}
