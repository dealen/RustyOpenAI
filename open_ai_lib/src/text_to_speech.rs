pub mod speech {
    use std::{env, fs::File, io::Write};

    use chrono::Utc;
    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
    use serde_json::json;

    pub struct Speech {
        pub open_ai_key: String,
        pub model: String,
    }

    impl Speech {
        #[must_use]
        pub fn new(open_ai_key: String) -> Speech {
            Speech {
                open_ai_key,
                model: "tts-1-hd".to_owned(),
            }
        }

        fn get_file_path(path: &str) -> String {
            let mut full_path = env::current_dir().unwrap();
            full_path.push(path);
            full_path.to_str().unwrap().to_string()
        }

        fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self.open_ai_key)
        }

        /// # Panics
        /// Panics if the no api key is provided or there is no connection
        /// # Errors
        /// Returns an error if the request fails
        pub async fn get_audio(&self, input: &str, voice: &str) {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();

            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&self.get_bearer_key()).expect("Could not create header"),
            );

            let data = json!({
                "model": self.model,
                "input": input,
                "voice": voice.to_lowercase()
            });

            let res = client
                .post("https://api.openai.com/v1/audio/speech")
                .headers(headers)
                .json(&data)
                .send()
                .await
                .expect("Could not get audio response");

            let body = res.bytes().await.unwrap();

            println!("Audio response received");

            Self::save_file_to_disk(&body);
        }

        fn save_file_to_disk(file_content: &[u8]) -> String {
            let path = format!("speech-{}.mp3", Utc::now().timestamp());
            let path_clone = path.clone();
            let mut file = File::create(path).unwrap();
            file.write_all(file_content).unwrap();

            // return path on disk
            Self::get_file_path(&path_clone)
        }
    }
}
