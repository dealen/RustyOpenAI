pub mod speech {
    use std::{
        env,
        fs::File,
        io::Write,
    };

    use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
    use serde_json::json;

    pub struct Speech {
        pub _open_ai_key: String,
        pub _model: String,
    }

    impl Speech {
        pub fn new(open_ai_key: String) -> Speech {
            Speech {
                _open_ai_key: open_ai_key,
                _model: "tts-1".to_owned(),
            }
        }

        fn get_file_path(path: &str) -> String {
            let mut full_path = env::current_dir().unwrap();
            full_path.push(path);
            full_path.to_str().unwrap().to_string()
        }

        fn save_file_to_disk(&self, file_content: String) -> String {
            let path = "speech.mp3";
            let mut file = File::create(path).unwrap();
            file.write_all(file_content.as_bytes()).unwrap();

            // return path on disk
            Self::get_file_path(path)
        }

        fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self._open_ai_key)
        }

        /*
            This is a sample call using curl from documentation:
            curl https://api.openai.com/v1/audio/speech \
        -H "Authorization: Bearer $OPENAI_API_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "model": "tts-1",
            "input": "Today is a wonderful day to build something people love!",
            "voice": "alloy"
        }' \
        --output speech.mp3
            */

        pub async fn get_audio(&self) {
            let client = reqwest::Client::new();
            let mut headers = HeaderMap::new();

            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&self.get_bearer_key()).expect("Could not create header"),
            );

            let data = json!({
                "model": self._model,
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

            println!("Audio response: {:?}", body);

            self.save_file_to_disk(body);
        }
        
    }

}
