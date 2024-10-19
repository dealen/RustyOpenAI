pub mod chat {
    use actix_web::{error, Error, HttpResponse, web};
    use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
    use serde_json::json;

    pub struct Chat {
        pub _open_ai_key: String,
        pub _model: String,
        pub _system_message: String,
        pub _message: String,
    }

    impl Chat {
        pub fn new(open_ai_key: String, model: String) -> Chat {
            Chat {
                _open_ai_key: open_ai_key,
                _model: model,
                _system_message: "".to_string(),
                _message: "".to_string(),
            }
        }

        fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self._open_ai_key)
        }

        pub fn get_model(&self) -> String {
            self._model.clone()
        }

        pub async fn ask_ai(&self, req: web::Path<String>) -> actix_web::Result<HttpResponse> {
            let message = req.into_inner();
            let bearer = self.get_bearer_key();
            let client = reqwest::Client::new();

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(AUTHORIZATION, bearer.parse().unwrap());

            let data = json!({
                "model": self.get_model(),
                "messages": [
                    {
                        "role": "system",
                        "content": "You are a helpful assistant."
                    },
                    {
                        "role": "user",
                        "content": message
                    }
                ],
                "stream": true
            });

            let res = client.post("https://api.openai.com/v1/chat/completions")
                .headers(headers)
                .json(&data)
                .send()
                .await
                .map_err(error::ErrorBadRequest)?;

            let response_text = res.text().await.map_err(error::ErrorBadRequest)?;

            Ok(HttpResponse::Ok().body(response_text))
        }

        pub fn add_system_message(&mut self, message: String) {
            let previous = self._system_message.to_string();
            self._system_message = format!("{}\n{}", previous, message);
        }

        pub fn add_user_message(&mut self, message: String) {
            let previous = self._message.to_string();
            self._message = format!("{}\n{}", previous, message);
        }

        pub async fn perform_conversation(&self, message: String) -> Result<String, Error>{
            let url = "https://api.openai.com/v1/chat/completions";
            let bearer = self.get_bearer_key();
            let client = reqwest::Client::new();

            let mut headers = HeaderMap::new();
            headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
            headers.insert(AUTHORIZATION, bearer.parse().unwrap());

            let data = json!({
                "model": self.get_model(),
                "messages": [
                    {
                        "role": "system",
                        "content": self._system_message.to_string()
                    },
                    {
                        "role": "user",
                        "content": self._message.to_string()
                    },
                    { // this should keep whole conversation instead of just user messages, I guess I should use assistamt role also here
                        "role": "user",
                        "content": message.to_string()
                    },
                ],
                "stream": false
            });

            let res = client.post(url.to_string())
                .headers(headers)
                .json(&data)
                .send()
                .await
                .map_err(error::ErrorBadRequest)?;

            Ok(res.text().await.map_err(error::ErrorBadRequest)?)
        }
    }
}