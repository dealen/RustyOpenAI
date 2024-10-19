pub mod speech {
    use std::{
        env,
        fs::File,
        io::Write,
    };

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
                model: "tts-1".to_owned(),
            }
        }

        fn get_file_path(path: &str) -> String {
            let mut full_path = env::current_dir().unwrap();
            full_path.push(path);
            full_path.to_str().unwrap().to_string()
        }

        fn save_file_to_disk(file_content: &str) -> String {
            let path = "speech.mp3";
            let mut file = File::create(path).unwrap();
            file.write_all(file_content.as_bytes()).unwrap();

            // return path on disk
            Self::get_file_path(path)
        }

        fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self.open_ai_key)
        }

        /// # Panics
        /// Panics if the no api key is provided or there is no connection
        /// # Errors
        /// Returns an error if the request fails
        pub async fn get_audio(&self) {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();

            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&self.get_bearer_key()).expect("Could not create header"),
            );

            let data = json!({
                "model": self.model,
                "input": "Today is a wonderful day to build something people love!",
                "voice": "alloy"
            });

            let res = client
                .post("https://api.openai.com/v1/audio/speech")
                .headers(headers)
                .json(&data)
                .send()
                .await
                .expect("Could not get audio response");

            let body = res.text().await.unwrap();

            println!("Audio response: {body:?}");

            Self::save_file_to_disk(&body);
        }
        
    }

}
