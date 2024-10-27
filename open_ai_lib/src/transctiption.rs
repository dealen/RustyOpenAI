use std::fs::File;
use std::io::Read;
use reqwest::{multipart, Client};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

pub struct Transcription {
    pub api_key: String,
}

impl Transcription {
    #[must_use] pub fn new(api_key: String) -> Transcription {
        Transcription {
            api_key,
        }
    }

    #[must_use] pub fn get_bearer_key(&self) -> String {
        format!("Bearer {}", self.api_key)
    }

    /// # Panics
    /// 
    /// Panics if the no api key is provided or there is no connection
    /// # Errors
    /// 
    /// Returns an error if the request fails
    pub async fn transcript(&self, path_to_audio_file: &str) -> Result<String, Box<dyn std::error::Error>>
    {
        let client = Client::new();
        let mut headers = HeaderMap::new();

        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&self.get_bearer_key()).expect("Could not create header"),
        );

        let mut file = File::open(path_to_audio_file).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();

        let form = multipart::Form::new()
            .text("model", "whisper-1")
            .file("file", path_to_audio_file).await?;

        let res = client
            .post("https://api.openai.com/v1/audio/transcriptions")
            .headers(headers)
            .multipart(form)
            .send()
            .await?;

        println!("Transcription response received");

        let status = res.status().as_u16();
        println!("Status: {status:?}");
        let body = res.text().await.unwrap();

        match status {
            200 => Ok(body),
            _ => Err(format!("Error: {body:?}").into()),
        }
    }
}