pub mod speech {

    pub struct Speech {
        pub _open_ai_key: String,
        pub _model: String,
    }

    impl Speech {
        pub fn new(open_ai_key: String) -> Speech {
            Speech {
                _open_ai_key: open_ai_key,
                _model: "tts-1",
            }
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
        pub fn get_audio() -> String {
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

            save_file_to_disk(body);
        }

        fn save_file_to_disk(&self, file: String) -> String {
            let mut file = File::create("speech.mp3").unwrap();
            file.write_all(file.as_bytes()).unwrap();

            // return path on disk
            file.path()
        }
    }
}