use actix_web::{error, web, App, HttpResponse, HttpServer, Responder, Result, FromRequest};

pub mod chat {
    use actix_web::{dev, error, Error, FromRequest, HttpRequest, HttpResponse, web};
    use actix_web::http::header::{AUTHORIZATION, CONTENT_TYPE};
    use reqwest::header::HeaderMap;
    use serde_json::json;

    pub struct Chat {
        pub _open_ai_key: String,
        pub _model: String,
    }

    impl Chat {
        pub fn new(open_ai_key: String, model: String) -> Chat {
            Chat {
                _open_ai_key: open_ai_key,
                _model: model,
            }
        }

        fn get_bearer_key(&self) -> String {
            format!("Bearer {}", self._open_ai_key)
        }

        fn get_model(&self) -> String {
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
                "model": self.get_model(),//"gpt-3.5-turbo",
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
    }
}
