pub mod chat {
    use actix_web::{error, Error, HttpResponse, web};
    use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
    use serde_json::json;

    pub struct Chat {
        pub open_ai_key: String,
        pub model: String,
        pub system_message: String,
        pub message: String,
    }

    impl Chat {
        #[must_use]
        pub fn new(open_ai_key: String, model: String) -> Chat {
            Chat {
                open_ai_key,
                model,
                system_message: String::new(),
                message: String::new(),
            }
        }

        fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self.open_ai_key)
        }

        #[must_use]
        pub fn get_model(&self) -> String {
            self.model.clone()
        }

        /// # Panics
        /// Panics if the no api key is provided or there is no connection
        /// # Errors
        /// Returns an error if the request fails
        pub async fn ask_ai(&self, request: web::Path<String>) -> actix_web::Result<HttpResponse> {
            let message = request.into_inner();
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

            let completion_response = client.post("https://api.openai.com/v1/chat/completions")
                .headers(headers)
                .json(&data)
                .send()
                .await
                .map_err(error::ErrorBadRequest)?;

            let response_text = completion_response.text().await.map_err(error::ErrorBadRequest)?;

            Ok(HttpResponse::Ok().body(response_text))
        }

        pub fn add_system_message(&mut self, message: &str) {
            let previous = self.system_message.to_string();
            self.system_message = format!("{previous}\n{message}" );
        }

        pub fn add_user_message(&mut self, message: &str) {
            let previous = self.message.to_string();
            self.message = format!("{previous}\n{message}");
        }

        /// # Errors
        /// Returns an error if the request fails
        /// # Panics
        /// Panics if the no api key is provided or there is no connection
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
                        "content": self.system_message.to_string()
                    },
                    {
                        "role": "user",
                        "content": self.message.to_string()
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

            res.text().await.map_err(error::ErrorBadRequest)
        }
    }
}