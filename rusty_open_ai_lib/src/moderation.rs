pub mod moderation {
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
    use serde_json::json;

    pub struct Moderation { }

    impl Moderation {
        pub fn new() -> Moderation {
            Moderation { }
        }

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

            let res = client.post("https://api.openai.com/v1/moderations")
                .headers(headers)
                .json(&data)
                .send()
                .await
                .unwrap();

            let body = res.text().await.unwrap();

            let flagged = body
                .split("flagged\": ")
                .collect::<Vec<&str>>()[1]
                .split(",")
                .collect::<Vec<&str>>()[0]
                .to_string();

            flagged == "true"
        }
    }
}