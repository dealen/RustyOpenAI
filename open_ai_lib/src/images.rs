use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use std::{fs::File, io::Write};

use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use crate::helper;

#[derive(Deserialize)]
struct ImageResponse {
    data: Vec<ImageData>,
}

#[derive(Deserialize)]
struct ImageData {
    b64_json: String,
}

#[derive(Default)]
pub struct Images {
    pub open_ai_key: String,
    pub model: String,
}

impl Images {
    #[must_use]
    pub fn new(open_ai_key: String) -> Images {
        Images {
            open_ai_key,
            model: "dall-e-3".to_owned(),
        }
    }

    fn get_bearer_key(&self) -> String {
        format!("Bearer {}", self.open_ai_key)
    }

    fn save_image_to_disk(file_content: &str) -> String {
        let path = format!("image-{}.png", Utc::now().timestamp());
        let path_clone = path.clone();

        let mut file = File::create(&path).unwrap();
        let decoded_content = general_purpose::STANDARD.decode(file_content).unwrap();
        file.write_all(&decoded_content).unwrap();

        helper::get_file_path(&path_clone)
    }

    /// # Panics
    ///
    /// Panics if the no api key is provided or there is no connection
    /// # Errors
    ///
    /// Returns an error if the request fails
    pub async fn get_image(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let api_key = self.get_bearer_key();
        let client = Client::new();

        let request_body = json!({
            "model": self.model,
            "prompt": prompt.to_string(),
            "n": 1,
            "size": "1024x1024",
            "response_format": "b64_json",
        });

        let response = client
            .post("https://api.openai.com/v1/images/generations")
            .header("Content-Type", "application/json")
            .header("Authorization", api_key)
            .json(&request_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        println!("Raw response: {response_text}");

        let response_body: ImageResponse = serde_json::from_str(&response_text)?;
        let base64_image = &response_body.data[0].b64_json;

        let image_path = Self::save_image_to_disk(base64_image);

        Ok(image_path)
    }
}
